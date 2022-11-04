use crate::handlers::push_message::MessagePayload;
use crate::providers::PushProvider;
use a2::NotificationBuilder;
use async_trait::async_trait;
use std::io::Read;
use tracing::span;

#[derive(Debug, Clone)]
pub struct ApnsProvider {
    client: a2::Client,
    topic: String,
}

impl ApnsProvider {
    pub fn new_cert<R>(
        cert: &mut R,
        password: String,
        endpoint: a2::Endpoint,
        topic: String,
    ) -> crate::error::Result<Self>
    where
        R: Read,
    {
        Ok(ApnsProvider {
            client: a2::Client::certificate(cert, password.as_str(), endpoint)?,
            topic,
        })
    }
}

#[async_trait]
impl PushProvider for ApnsProvider {
    async fn send_notification(
        &mut self,
        token: String,
        payload: MessagePayload,
    ) -> crate::error::Result<()> {
        let s = span!(tracing::Level::DEBUG, "send_apns_notification");
        let _ = s.enter();

        let mut opt = a2::NotificationOptions::default();
        opt.apns_topic = Some(&self.topic);

        // TODO set title
        let notification =
            a2::PlainNotificationBuilder::new(&payload.description).build(token.as_str(), opt);

        let _ = self.client.send(notification).await?;

        Ok(())
    }
}
