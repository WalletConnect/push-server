use crate::handlers::push_message::MessagePayload;
use crate::providers::PushProvider;
use async_trait::async_trait;
use fcm::{MessageBuilder, NotificationBuilder};
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

#[async_trait]
impl PushProvider for FcmProvider {
    async fn send_notification(
        &mut self,
        token: String,
        payload: MessagePayload,
    ) -> crate::error::Result<()> {
        let mut builder = NotificationBuilder::new();
        builder.title(&payload.title);
        builder.body(&payload.description);
        let notification = builder.finalize();

        let mut builder = MessageBuilder::new(self.api_key.as_str(), token.as_str());
        builder.notification(notification);
        let fcm_message = builder.finalize();

        let _ = self.client.send(fcm_message).await?;

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
}

impl Debug for FcmProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[FcmProvider] api_key = {}", self.api_key)
    }
}
