use {
    crate::{error, error::Error::InvalidConfiguration, providers::ProviderKind},
    serde::Deserialize,
};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_log_level_otel")]
    pub log_level_otel: String,
    #[serde(default = "default_disable_header")]
    pub disable_header: bool,
    #[serde(default = "default_relay_url")]
    pub relay_url: String,
    #[serde(default = "default_validate_signatures")]
    pub validate_signatures: bool,
    pub database_url: String,
    pub tenant_database_url: Option<String>,
    #[serde(default = "default_tenant_id")]
    pub default_tenant_id: String,
    #[serde(default = "default_is_test", skip)]
    /// This is an internal flag to disable logging, cannot be defined by user
    pub is_test: bool,

    // TELEMETRY
    pub otel_exporter_otlp_endpoint: Option<String>,
    pub telemetry_prometheus_port: Option<u16>,

    // APNS
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

    pub fn single_tenant_supported_providers(&self) -> Vec<ProviderKind> {
        let mut supported = vec![];

        if self.apns_certificate.is_some() && self.apns_certificate_password.is_some() {
            supported.push(ProviderKind::Apns(true));
            supported.push(ProviderKind::Apns(false));
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

fn default_log_level() -> String {
    "info,echo-server=info".to_string()
}

fn default_log_level_otel() -> String {
    "info,echo-server=trace".to_string()
}

fn default_disable_header() -> bool {
    false
}

fn default_validate_signatures() -> bool {
    true
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
