use crate::error;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub telemetry_enabled: Option<bool>,
    pub telemetry_grpc_url: Option<String>,
}

impl Config {
    pub fn log_level(&self) -> tracing::Level {
        tracing::Level::from_str(self.log_level.as_str()).expect("Invalid log level")
    }
}

fn default_port() -> u16 {
    3000
}

fn default_log_level() -> String {
    "WARN".to_string()
}

pub fn get_config() -> error::Result<Config> {
    let config = envy::from_env::<Config>()?;
    Ok(config)
}
