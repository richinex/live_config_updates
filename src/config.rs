use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub feature_flag: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            feature_flag: false, // Or any other default values
        }
    }
}



impl Config {
    pub fn load_from_file(file_path: &str) -> Result<Self, serde_yaml::Error> {
        let mut file = File::open(file_path).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        serde_yaml::from_str(&contents)
    }
}
