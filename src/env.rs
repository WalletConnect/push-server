use serde::{Deserialize};
use crate::error;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    pub redis_url: String,
    pub telemetry_enabled: Option<bool>,
    pub telemetry_grpc_url: Option<String>,
}

fn default_port() -> u16 {
    3000
}

pub fn get_config() -> error::Result<Config> {
    let config = envy::from_env::<Config>()?;
    Ok(config)
}