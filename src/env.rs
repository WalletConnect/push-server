use serde::{Deserialize};
use crate::error;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16
}

fn default_port() -> u16 {
    3000
}

pub fn get_config() -> error::Result<Config> {
    let config = envy::from_env::<Config>()?;
    Ok(config)
}