pub mod apns;
pub mod fcm;
pub mod noop;

use crate::handlers::push_message::MessagePayload;
use crate::providers::noop::NoopProvider;
use crate::store::ClientStore;
use crate::{env::Config, error::Error::ProviderNotAvailable};
use crate::{error, providers::fcm::FcmProvider};
use crate::{providers::apns::ApnsProvider, state::AppState};
use async_trait::async_trait;
use std::fs::File;
use std::io::BufReader;

#[async_trait]
pub trait PushProvider {
    async fn send_notification(
        &mut self,
        token: String,
        payload: MessagePayload,
    ) -> crate::error::Result<()>;
}

const PROVIDER_APNS: &str = "apns";
const PROVIDER_FCM: &str = "fcm";
const PROVIDER_NOOP: &str = "noop";

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "provider")]
#[sqlx(rename_all = "lowercase")]
pub enum ProviderKind {
    Apns,
    Fcm,
    Noop,
}

impl ProviderKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Apns => PROVIDER_APNS,
            Self::Fcm => PROVIDER_FCM,
            Self::Noop => PROVIDER_NOOP,
        }
    }
}

impl From<ProviderKind> for &str {
    fn from(val: ProviderKind) -> Self {
        val.as_str()
    }
}

impl TryFrom<&str> for ProviderKind {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            PROVIDER_APNS => Ok(Self::Apns),
            PROVIDER_FCM => Ok(Self::Fcm),
            PROVIDER_NOOP => Ok(Self::Noop),
            _ => Err(error::Error::ProviderNotFound(value.to_owned())),
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum Provider {
    Fcm(FcmProvider),
    Apns(ApnsProvider),
    Noop(NoopProvider),
}

#[async_trait]
impl PushProvider for Provider {
    async fn send_notification(
        &mut self,
        token: String,
        payload: MessagePayload,
    ) -> crate::error::Result<()> {
        match self {
            Provider::Fcm(p) => p.send_notification(token, payload).await,
            Provider::Apns(p) => p.send_notification(token, payload).await,
            Provider::Noop(p) => p.send_notification(token, payload).await,
        }
    }
}

pub struct Providers {
    apns: Option<ApnsProvider>,
    fcm: Option<FcmProvider>,
    noop: NoopProvider,
}

impl Providers {
    pub fn new(config: &Config) -> crate::error::Result<Providers> {
        let supported = config.supported_providers();
        let mut apns = None;
        if supported.contains(&ProviderKind::Apns) {
            // Certificate Based
            if let Some(cert_config) = &config.apns_certificate {
                let f = File::open(cert_config.cert_path.clone())?;
                let mut reader = BufReader::new(f);

                let mut endpoint = a2::Endpoint::Production;
                if let Some(sandbox) = cert_config.sandbox {
                    if sandbox {
                        endpoint = a2::Endpoint::Sandbox;
                    }
                }

                apns = Some(ApnsProvider::new_cert(
                    &mut reader,
                    cert_config.password.clone(),
                    endpoint,
                )?);
            }

            // Token Based
            if let Some(token_config) = &config.apns_token {
                let f = File::open(token_config.token_path.clone())?;
                let mut reader = BufReader::new(f);

                let mut endpoint = a2::Endpoint::Production;
                if let Some(sandbox) = token_config.sandbox {
                    if sandbox {
                        endpoint = a2::Endpoint::Sandbox;
                    }
                }

                apns = Some(ApnsProvider::new_token(
                    &mut reader,
                    token_config.key_id.clone(),
                    token_config.team_id.clone(),
                    endpoint,
                )?);
            }
        }

        let mut fcm = None;
        if supported.contains(&ProviderKind::Fcm) {
            if let Some(api_key) = &config.fcm_api_key {
                fcm = Some(FcmProvider::new(api_key.clone()))
            }
        }

        Ok(Providers {
            apns,
            fcm,
            noop: NoopProvider::new(),
        })
    }
}

pub fn get_provider(
    provider: ProviderKind,
    state: &AppState<impl ClientStore>,
) -> crate::error::Result<Provider> {
    let name = provider.as_str();
    let supported = state.config.supported_providers();

    if !supported.contains(&provider) {
        return Err(ProviderNotAvailable(name.into()));
    }

    match provider {
        ProviderKind::Apns => match state.providers.apns.clone() {
            Some(p) => Ok(Provider::Apns(p)),
            None => Err(ProviderNotAvailable(name.into())),
        },
        ProviderKind::Fcm => match state.providers.fcm.clone() {
            Some(p) => Ok(Provider::Fcm(p)),
            None => Err(ProviderNotAvailable(name.into())),
        },
        ProviderKind::Noop => {
            // Only available in debug/testing
            if cfg!(any(test, debug_assertions)) {
                return Ok(Provider::Noop(state.providers.noop.clone()));
            }

            Err(ProviderNotAvailable(name.into()))
        }
    }
}
