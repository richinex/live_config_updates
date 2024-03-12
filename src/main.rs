use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

mod config;
mod publisher;
mod subscriber;

use config::{Config, ConfigUpdate};
use publisher::Publisher;
use subscriber::listen_for_updates;

use slog::{Drain, Logger, o};
use slog_async::Async;
use slog_json::Json;
use slog_term::{FullFormat, TermDecorator};

fn configure_logging() -> Logger {
    let decorator = TermDecorator::new().build();
    let console_drain = FullFormat::new(decorator).build().fuse();
    let console_drain = Async::new(console_drain).build().fuse();

    let json_drain = Json::new(std::io::stdout())
        .add_default_keys()
        .build().fuse();
    let json_drain = Async::new(json_drain).build().fuse();

    Logger::root(slog::Duplicate::new(console_drain, json_drain).fuse(), o!())
}

// Endpoint for updating configuration with partial updates
async fn update_config_endpoint(
    publisher: web::Data<Arc<Mutex<Publisher>>>,
    updates: web::Json<ConfigUpdate>, // Accept ConfigUpdate
    // No need to explicitly pass the logger here if it's part of the Publisher state
) -> impl Responder {
    let publisher = publisher.lock().unwrap();
    publisher.update_config_with_updates(updates.into_inner()); // Logger is already part of Publisher
    HttpResponse::Ok().body("Configuration updated")
}


// Endpoint to get the current configuration
async fn get_config_endpoint(publisher: web::Data<Arc<Mutex<Publisher>>>, logger: web::Data<Logger>) -> impl Responder {
    let publisher = publisher.lock().unwrap();
    let config = Arc::clone(&publisher.get_current_config());
    slog::info!(logger.get_ref(), "Fetching current configuration"; "config" => format!("{:?}", &*config));
    HttpResponse::Ok().json(&*config)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let root_logger = configure_logging();
    slog::info!(root_logger, "Starting HTTP server for configuration management");

    let initial_config = Config::load_from_file("config.yml")
        .expect("Failed to load initial configuration from file");
    let (tx, rx) = watch::channel(initial_config);
    let publisher = Arc::new(Mutex::new(Publisher::new(tx, Some(root_logger.clone())))); // Clone logger here

    // Clone the logger before passing to tokio::spawn to retain ownership for later use
    let subscriber_logger = root_logger.clone();
    tokio::spawn(async move {
        listen_for_updates(rx, subscriber_logger).await; // Use cloned logger
    });

    HttpServer::new(move || {
        App::new()
            // .wrap(ActixLogger::default()) // Optionally use Actix Logger middleware for request logging
            .app_data(web::Data::new(publisher.clone()))
            // Clone the logger again for use in HTTP server setup
            .app_data(web::Data::new(root_logger.clone()))
            .route("/update-config", web::post().to(update_config_endpoint))
            .route("/config", web::get().to(get_config_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

