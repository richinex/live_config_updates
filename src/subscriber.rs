// Import necessary modules for asynchronous communication and application configuration structure.
use tokio::sync::watch::Receiver;
use crate::config::Config;

// Import the logging interface.
use log::info;

// Define an asynchronous function to listen for configuration updates.
pub async fn listen_for_updates(mut receiver: Receiver<Config>) {
    // Infinite loop to continuously listen for updates.
    loop {
        // Attempt to receive the next configuration update. Await here pauses the execution
        // until a new configuration is sent or until all Sender halves are dropped, in which case it errors.
        if receiver.changed().await.is_err() {
            // If an error occurs (e.g., all Sender halves are dropped indicating no more updates will be sent),
            // log a message indicating that the configuration channel has been closed and exit the loop.
            info!("Configuration channel closed.");
            break;
        }
        // If `changed().await` did not error, it means a new configuration is available.
        // Access the most recent configuration sent over the channel.
        // `.borrow()` gets a reference to the value in the channel without cloning the data.
        let config = receiver.borrow();

        // Log the newly received configuration for visibility.
        // The dereference operator `*` is used to get the value from the reference returned by `.borrow()`.
        info!("Received updated config: {:?}", *config);

        // Here, you would typically apply the configuration changes to your application.
        // This might involve adjusting application behavior, toggling features, etc., based on the new config.
    }
}
