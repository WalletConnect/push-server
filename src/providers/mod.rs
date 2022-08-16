mod apns;
mod apns_test;
mod fcm;
mod fcm_test;
mod noop;
mod noop_test;

use crate::error::Error::{ProviderNotAvailable, ProviderNotFound};
use crate::providers::apns::ApnsProvider;
use crate::providers::fcm::FcmProvider;
use crate::providers::noop::NoopProvider;
use crate::store::ClientStore;
use crate::{Config, State};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub trait PushProvider {
    fn send_notification(&mut self, token: String, message: String) -> crate::error::Result<()>;
}

pub enum Provider {
    Fcm(FcmProvider),
    Apns(ApnsProvider),
    Noop(NoopProvider),
}

impl PushProvider for Provider {
    fn send_notification(&mut self, token: String, message: String) -> crate::error::Result<()> {
        match self {
            Provider::Fcm(p) => p.send_notification(token, message),
            Provider::Apns(p) => p.send_notification(token, message),
            Provider::Noop(p) => p.send_notification(token, message),
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
        if supported.contains(&"apns".to_string()) {
            // Certificate Based
            if let Some(cert_config) = &config.apns_certificate {
                let f = File::open(cert_config.cert_path.clone())?;
                let mut reader = BufReader::new(f);

                let mut endpoint = a2::Endpoint::Production;
                if let Some(sandbox) = cert_config.sandbox.clone() {
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
                if let Some(sandbox) = token_config.sandbox.clone() {
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
        if supported.contains(&"fcm".to_string()) {
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
    name: String,
    state: &Arc<State<impl ClientStore>>,
) -> crate::error::Result<Provider> {
    let supported = state.config.supported_providers();

    if !supported.contains(&name.to_lowercase()) {
        return Err(ProviderNotAvailable(name));
    }

    match name.as_str() {
        "apns" => match state.providers.apns.clone() {
            Some(p) => Ok(Provider::Apns(p)),
            None => Err(ProviderNotAvailable(name)),
        },
        "fcm" => match state.providers.fcm.clone() {
            Some(p) => Ok(Provider::Fcm(p)),
            None => Err(ProviderNotAvailable(name)),
        },
        "noop" => {
            // Only available in debug/testing
            if cfg!(any(test, debug_assertions)) {
                return Ok(Provider::Noop(state.providers.noop.clone()));
            }

            Err(ProviderNotAvailable(name))
        }
        _ => Err(ProviderNotFound(name)),
    }
}
