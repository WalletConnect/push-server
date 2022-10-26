use crate::{error, providers::ProviderKind};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub database_url: String,

    // TELEMETRY
    pub telemetry_enabled: Option<bool>,
    pub telemetry_grpc_url: Option<String>,

    // APNS
    /// This defaults to false and should be changed if sandbox mode is required.
    #[serde(default = "default_apns_sandbox_mode")]
    pub apns_sandbox: bool,
    pub apns_certificate: Option<String>,
    pub apns_certificate_password: Option<String>,
    pub apns_topic: Option<String>,

    // FCM
    pub fcm_api_key: Option<String>,
}

impl Config {
    pub fn log_level(&self) -> tracing::Level {
        tracing::Level::from_str(self.log_level.as_str()).expect("Invalid log level")
    }

    pub fn supported_providers(&self) -> Vec<ProviderKind> {
        let mut supported = vec![];

        if self.apns_certificate.is_some() && self.apns_certificate_password.is_some() {
            supported.push(ProviderKind::Apns);
        }

        if self.fcm_api_key.is_some() {
            supported.push(ProviderKind::Fcm);
        }

        // Only available in debug/testing
        #[cfg(any(debug_assertions, test))]
        supported.push(ProviderKind::Noop);

        supported
    }
}

fn default_port() -> u16 {
    3000
}

fn default_log_level() -> String {
    "WARN".to_string()
}

fn default_apns_sandbox_mode() -> bool {
    false
}

pub fn get_config() -> error::Result<Config> {
    let config = envy::from_env::<Config>()?;
    Ok(config)
}
