use a2::NotificationOptions;
use {
    crate::{handlers::push_message::MessagePayload, providers::PushProvider},
    a2::NotificationBuilder,
    async_trait::async_trait,
    std::io::Read,
    tracing::span,
};

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

        let opt = NotificationOptions {
            apns_id: None,
            apns_expiration: None,
            apns_priority: None,
            apns_topic: Some(&self.topic),
            apns_collapse_id: None
        };

        // TODO set title
        let notification =
            a2::PlainNotificationBuilder::new(&payload.description).build(token.as_str(), opt);

        let _ = self.client.send(notification).await?;

        Ok(())
    }
}
