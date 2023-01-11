use {
    crate::{error, error::Error::InvalidConfiguration, providers::ProviderKind},
    serde::Deserialize,
    std::str::FromStr,
};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_log_level_otel")]
    pub log_level_otel: String,
    #[serde(default = "default_relay_url")]
    pub relay_url: String,
    pub database_url: String,
    pub tenant_database_url: Option<String>,
    #[serde(default = "default_tenant_id")]
    pub default_tenant_id: String,
    #[serde(default = "default_is_test", skip)]
    /// This is an internal flag to disable logging, cannot be defined by user
    pub is_test: bool,

    // TELEMETRY
    pub telemetry_enabled: Option<bool>,
    pub telemetry_grpc_url: Option<String>,
    #[serde(default = "default_telemetry_prometheus_port")]
    pub telemetry_prometheus_port: u16,

    // APNS
    /// This defaults to false and should be changed if sandbox mode is
    /// required.
    #[serde(default = "default_apns_sandbox_mode")]
    pub apns_sandbox: bool,
    pub apns_certificate: Option<String>,
    pub apns_certificate_password: Option<String>,
    pub apns_topic: Option<String>,

    // FCM
    pub fcm_api_key: Option<String>,
}

impl Config {
    /// Run validations against config and throw error
    pub fn is_valid(&self) -> error::Result<()> {
        if self.tenant_database_url.is_none() && self.single_tenant_supported_providers().is_empty()
        {
            return Err(InvalidConfiguration(
                "no tenant database url provided and no provider keys found".to_string(),
            ));
        }

        if !self.single_tenant_supported_providers().is_empty()
            && self.tenant_database_url.is_some()
        {
            return Err(InvalidConfiguration(
                "tenant database and providers keys found in config".to_string(),
            ));
        }

        Ok(())
    }

    pub fn log_level(&self) -> tracing::Level {
        tracing::Level::from_str(self.log_level.as_str()).expect("Invalid log level")
    }

    pub fn log_level_otel(&self) -> tracing::Level {
        tracing::Level::from_str(self.log_level_otel.as_str()).expect("Invalid log level")
    }

    pub fn single_tenant_supported_providers(&self) -> Vec<ProviderKind> {
        let mut supported = vec![];

        if self.apns_certificate.is_some() && self.apns_certificate_password.is_some() {
            supported.push(ProviderKind::Apns);
        }

        if self.fcm_api_key.is_some() {
            supported.push(ProviderKind::Fcm);
        }

        // Only available in debug/testing
        #[cfg(any(debug_assertions, test))]
        if self.tenant_database_url.is_none() {
            supported.push(ProviderKind::Noop);
        }

        supported
    }
}

fn default_port() -> u16 {
    3000
}

fn default_telemetry_prometheus_port() -> u16 {
    3001
}

fn default_log_level() -> String {
    "info,echo-server=info".to_string()
}

fn default_log_level_otel() -> String {
    "info,echo-server=trace".to_string()
}

fn default_apns_sandbox_mode() -> bool {
    false
}

fn default_relay_url() -> String {
    "https://relay.walletconnect.com".to_string()
}

fn default_tenant_id() -> String {
    "0000-0000-0000-0000".to_string()
}

fn default_is_test() -> bool {
    false
}

pub fn get_config() -> error::Result<Config> {
    let config = envy::from_env::<Config>()?;
    Ok(config)
}
