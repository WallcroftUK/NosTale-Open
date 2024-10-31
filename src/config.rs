use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config/server.toml")?; // Ensure you have a config.toml file
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
