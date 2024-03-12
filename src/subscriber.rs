use tokio::sync::watch::Receiver;
use crate::config::Config;
use slog::{Logger, info};
use chrono::Local;
use std::env;

pub async fn listen_for_updates(mut receiver: Receiver<Config>, logger: Logger) {
    let env = env::var("RUN_ENV").unwrap_or_else(|_| "unknown".into());
    let mut previous_config: Option<Config> = None;

    loop {
        if receiver.changed().await.is_err() {
            info!(logger, "Configuration channel closed"; "time" => Local::now().to_rfc3339(), "environment" => &env);
            break;
        }

        let current_config_ref = receiver.borrow();
        let current_config = (*current_config_ref).clone();

        if let Some(ref previous) = previous_config {
            if previous != &current_config {
                // Adjust the logging here to reflect the new ball properties in the Config struct
                info!(logger, "Configuration updated";
                    "time" => Local::now().to_rfc3339(),
                    "environment" => &env,
                    "ball_color" => &current_config.ball_color,
                    "ball_size" => current_config.ball_size,
                    "ball_speed" => current_config.ball_speed,
                    "number_of_balls" => current_config.number_of_balls,
                    "previous_ball_color" => &previous.ball_color,
                    "previous_ball_size" => previous.ball_size,
                    "previous_ball_speed" => previous.ball_speed,
                    "previous_number_of_balls" => previous.number_of_balls
                );
            }
        } else {
            info!(logger, "Initial configuration received";
                "time" => Local::now().to_rfc3339(),
                "environment" => &env,
                "config" => format!("{:?}", current_config) // This will automatically use the Debug impl of Config
            );
        }

        previous_config = Some(current_config);
    }
}
