use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

use crate::config::config::Config;
use crate::config::config_adapter::ConfigAdapter;

#[derive(Debug)]
pub struct JsonConfigAdapter {}

impl ConfigAdapter for JsonConfigAdapter {
    fn new() -> Self {
        JsonConfigAdapter {}
    }

    fn load(&self, path: &str) -> Result<Config, String> {
        println!("loading");
        let file_path = Path::new(path);
        if file_path.is_file() {
            println!("Config is loaded from: {}", path);
            let config_json = fs::read_to_string(file_path).expect("Unable to read file");

            let json: Config = serde_json::from_str(config_json.as_str())
                .expect("Config JSON was not well-formatted");
            return Ok(json);
        } else {
            print!("Your config source path is invalid: {}", path);
        }
        return Err("failed to load config".to_string());
    }
}
