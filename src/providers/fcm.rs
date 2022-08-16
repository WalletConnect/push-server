use crate::providers::PushProvider;
use std::fmt::{Debug, Formatter};

pub struct FcmProvider {
    api_key: String,
    client: fcm::Client,
}

impl FcmProvider {
    pub fn new(api_key: String) -> Self {
        FcmProvider {
            api_key,
            client: fcm::Client::new(),
        }
    }
}

impl PushProvider for FcmProvider {
    fn send_notification(&mut self, token: String, message: String) -> crate::error::Result<()> {
        let mut notification_builder = fcm::NotificationBuilder::new().body(&message);
        let notification = notification_builder.finalize();

        let mut message_builder = fcm::MessageBuilder::new(&self.api_key, &token).notification(notification);
        let message = message_builder.finalize();

        let _res = self.client.send(message);

        Ok(())
    }
}

// Manual Impl Because `fcm::Client` does not derive anything and doesn't need to be accounted for

impl Clone for FcmProvider {
    fn clone(&self) -> Self {
        FcmProvider {
            api_key: self.api_key.clone(),
            client: fcm::Client::new(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.api_key = source.api_key.clone();
        self.client = fcm::Client::new();
    }
}

impl PartialEq for FcmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.api_key == other.api_key
    }

    fn ne(&self, other: &Self) -> bool {
        self.api_key != other.api_key
    }
}

impl Debug for FcmProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[FcmProvider] api_key = {}", self.api_key)
    }
}
