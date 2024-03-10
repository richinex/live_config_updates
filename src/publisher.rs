use tokio::sync::watch::Sender;
use crate::config::Config;
use std::sync::Arc;

// Define a struct for the Publisher, which is responsible for sending configuration updates.
pub struct Publisher {
    // Sender part of a `tokio::sync::watch` channel for sending configuration updates to subscribers.
    sender: Sender<Config>,
    // Arc for thread-safe reference counting. This allows the Publisher to share ownership
    // of the current configuration across multiple threads, ensuring safe concurrent access.
    current_config: Arc<Config>,
}

impl Publisher {
    // Constructor for the Publisher struct. Initializes a new Publisher with a given Sender
    // and the default configuration.
    pub fn new(sender: Sender<Config>) -> Self {
        Self {
            sender,
            // Wraps the default configuration in an Arc to enable shared ownership and thread-safe mutation.
            current_config: Arc::new(Config::default()),
        }
    }

    // Method to update the configuration. Takes a new configuration object as its parameter.
    pub fn update_config(&mut self, new_config: Config) {
        // Update the Publisher's current configuration to the new configuration,
        // wrapping it in a new Arc for shared ownership.
        self.current_config = Arc::new(new_config.clone());
        // Send the new configuration to all subscribers through the watch channel.
        // Uses `let _ =` to ignore the Result returned by `sender.send()`,
        // which would be an error if there are no subscribers listening.
        let _ = self.sender.send(new_config);
    }

    // Getter method for accessing the current configuration.
    // Returns an Arc pointing to the current configuration,
    // allowing multiple threads to safely access the configuration concurrently.
    pub fn get_current_config(&self) -> Arc<Config> {
        // Clones the Arc, increasing the reference count without cloning the underlying Config.
        // This allows the caller to obtain a reference to the current configuration
        // without taking ownership of it, maintaining shared access.
        Arc::clone(&self.current_config)
    }
}
