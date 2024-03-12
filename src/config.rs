use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};
use serde_yaml::Error as YamlError;

// Define a custom error type that covers different kinds of errors that could occur
#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    YamlError(YamlError),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::IoError(err)
    }
}

impl From<YamlError> for ConfigError {
    fn from(err: YamlError) -> ConfigError {
        ConfigError::YamlError(err)
    }
}


// Your main configuration struct with all the potential fields that can be configured.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub ball_color: String,    // Color of the balls (e.g., "green", "red", "blue")
    pub ball_size: u8,         // Diameter of the balls in pixels
    pub ball_speed: u8,        // Speed of the balls' movement (pixels per animation frame)
    pub number_of_balls: u8,   // Total number of balls to display
}


// Implementation block for Config, including loading from a file and a default configuration.
impl Config {
    // Loads a configuration from a YAML file.
    pub fn load_from_file(file_path: &str) -> Result<Self, ConfigError> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    // Merges updates from a ConfigUpdate instance into the current Config instance.
    pub fn merge(&mut self, updates: ConfigUpdate) {
        if let Some(ball_color) = updates.ball_color {
            self.ball_color = ball_color;
        }
        if let Some(ball_size) = updates.ball_size {
            self.ball_size = ball_size;
        }
        if let Some(ball_speed) = updates.ball_speed {
            self.ball_speed = ball_speed;
        }
        if let Some(number_of_balls) = updates.number_of_balls {
            self.number_of_balls = number_of_balls;
        }
    }
}

// Default implementation for Config, providing default values for all fields.
impl Default for Config {
    fn default() -> Self {
        Self {
            ball_color: "blue".to_string(),
            ball_size: 30,        // Example default size in pixels
            ball_speed: 5,        // Example default speed
            number_of_balls: 50,  // Example default number of balls
        }
    }
}
// A struct to represent potential updates to the configuration, with all fields optional.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigUpdate {
    pub ball_color: Option<String>,
    pub ball_size: Option<u8>,
    pub ball_speed: Option<u8>,
    pub number_of_balls: Option<u8>,
}