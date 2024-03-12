// use tokio::sync::watch::Sender;
// use crate::config::{Config, ConfigUpdate};
// use std::sync::{Arc, Mutex};

// pub struct Publisher {
//     sender: Sender<Config>,
//     current_config: Mutex<Arc<Config>>,
// }

// impl Publisher {
//     pub fn new(sender: Sender<Config>) -> Self {
//         Self {
//             sender,
//             current_config: Mutex::new(Arc::new(Config::default())),
//         }
//     }

//     // // Method to update the entire configuration
//     // pub fn update_config(&self, new_config: Config) {
//     //     let mut current_config = self.current_config.lock().unwrap();
//     //     *current_config = Arc::new(new_config.clone());
//     //     let _ = self.sender.send(new_config);
//     // }

//     // Method to apply partial updates to the current configuration
//     pub fn update_config_with_updates(&self, updates: ConfigUpdate) {
//         let mut current_config_lock = self.current_config.lock().unwrap();
//         let current_config = Arc::make_mut(&mut *current_config_lock);

//         // Here's where we use the merge method
//         current_config.merge(updates);

//         // After merging updates, broadcast the updated configuration to all subscribers
//         let _ = self.sender.send(current_config.clone());
//     }

//     pub fn get_current_config(&self) -> Arc<Config> {
//         let current_config = self.current_config.lock().unwrap();
//         Arc::clone(&*current_config)
//     }
// }
use tokio::sync::watch::Sender;
use crate::config::{Config, ConfigUpdate};
use std::sync::{Arc, Mutex};
use slog::Logger; // Import the Logger from slog

pub struct Publisher {
    sender: Sender<Config>,
    current_config: Mutex<Arc<Config>>,
    logger: Option<Logger>, // Optional Log
}

impl Publisher {
    pub fn new(sender: Sender<Config>, logger: Option<Logger>) -> Self {
        Self {
            sender,
            current_config: Mutex::new(Arc::new(Config::default())),
            logger,
        }
    }

    // Method to apply partial updates to the current configuration
    pub fn update_config_with_updates(&self, updates: ConfigUpdate) {
        let mut current_config_lock = self.current_config.lock().unwrap();
        let current_config = Arc::make_mut(&mut *current_config_lock);

        // Use the merge method to apply updates
        current_config.merge(updates);

        // Log the update using the provided logger
        if let Some(ref logger) = self.logger {
            slog::info!(logger, "Configuration updated"; "new_config" => ?current_config);
        }
        // Broadcast the updated configuration to all subscribers
        let _ = self.sender.send((*current_config).clone());
    }

    pub fn get_current_config(&self) -> Arc<Config> {
        let current_config = self.current_config.lock().unwrap();
        return Arc::clone(&*current_config);
    }
}
