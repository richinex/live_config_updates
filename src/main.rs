use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

mod config;
mod publisher;
mod subscriber;

use config::Config;
use publisher::Publisher;

async fn update_config_endpoint(publisher: web::Data<Arc<Mutex<Publisher>>>, config: web::Json<Config>) -> impl Responder {
    let mut publisher = publisher.lock().unwrap();
    publisher.update_config(config.into_inner());
    HttpResponse::Ok().body("Configuration updated")
}

async fn get_config(publisher: web::Data<Arc<Mutex<Publisher>>>) -> impl Responder {
    let publisher = publisher.lock().unwrap();
    // Dereference the Arc to get a reference to Config
    let config = Arc::clone(&publisher.get_current_config());
    HttpResponse::Ok().json(&*config) // Correctly dereference the Arc for serialization
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load initial configuration from a YAML file
    let initial_config = Config::load_from_file("config.yml")
        .expect("Failed to load initial configuration from file");
    let (tx, rx) = watch::channel(initial_config);
    let publisher = Arc::new(Mutex::new(Publisher::new(tx)));

    tokio::spawn(async move {
        subscriber::listen_for_updates(rx).await;
    });

    log::info!("Starting HTTP server for configuration management");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(publisher.clone()))
            .route("/update-config", web::post().to(update_config_endpoint))
            .route("/config", web::get().to(get_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
