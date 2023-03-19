// src/config/settings.rs
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub storage: StorageSettings,
    pub api_server: Option<ApiServerSettings>,
    // Add other settings as needed
}

#[derive(Debug, Deserialize)]
pub struct StorageSettings {
    pub data_directory: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiServerSettings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config_file = File::open("config.toml")?;
        let mut content = String::new();
        config_file.read_to_string(&mut content)?;

        let settings: Settings = toml::from_str(&content)?;

        Ok(settings)
    }
}
