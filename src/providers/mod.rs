mod apns;
mod fcm;

use crate::error::Error::ProviderNotFound;

pub trait PushProvider {
    fn send_notification(message: String) -> crate::error::Result<()>;
}

pub fn get_provider(name: String) -> crate::error::Result<impl PushProvider> {
    match name.as_str() {
        "apns" => Ok(apns::new()),
        "fcm" => Ok(fcm::new()),
        _ => Err(ProviderNotFound(name)),
    }
}
