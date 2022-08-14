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

pub fn get_provider(name: String) -> crate::error::Result<Provider> {
    match name.as_str() {
        "apns" => Ok(Provider::Apns(apns::new())),
        "fcm" => Ok(Provider::Fcm(fcm::new())),
        "noop" => {
            // Only available in debug/testing
            if cfg!(any(test, debug_assertions)) {
                return Ok(Provider::Noop(noop::new()));
            }

            Err(ProviderNotAvailable(name))
        }
        _ => Err(ProviderNotFound(name)),
    }
}
