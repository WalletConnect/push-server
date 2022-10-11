use crate::{error, providers::ProviderKind};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ApnsCertificateConfig {
    pub cert_path: String,
    pub password: String,
    pub sandbox: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ApnsTokenConfig {
    pub token_path: String,
    pub team_id: String,
    pub key_id: String,
    pub sandbox: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_database_url")]
    pub database_url: String,
    pub telemetry_enabled: Option<bool>,
    pub telemetry_grpc_url: Option<String>,
    pub apns_certificate: Option<ApnsCertificateConfig>,
    pub apns_token: Option<ApnsTokenConfig>,
    pub fcm_api_key: Option<String>,
}

impl Config {
    pub fn log_level(&self) -> tracing::Level {
        tracing::Level::from_str(self.log_level.as_str()).expect("Invalid log level")
    }

    pub fn supported_providers(&self) -> Vec<ProviderKind> {
        let mut supported = vec![];

        if self.apns_certificate.is_some() || self.apns_token.is_some() {
            supported.push(ProviderKind::Apns)
        }

        if self.fcm_api_key.is_some() {
            supported.push(ProviderKind::Fcm)
        }

        // Only available in debug/testing
        if cfg!(any(test, debug_assertions)) {
            supported.push(ProviderKind::Noop)
        }

        supported
    }
}

fn default_port() -> u16 {
    3000
}

fn default_log_level() -> String {
    "WARN".to_string()
}

fn default_database_url() -> String {
    "".to_string()
}

pub fn get_config() -> error::Result<Config> {
    let config = envy::from_env::<Config>()?;
    Ok(config)
}
