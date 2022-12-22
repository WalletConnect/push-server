use {
    crate::{
        handlers::push_message::{DecryptedPayloadBlob, MessagePayload},
        providers::PushProvider,
    },
    a2::{NotificationBuilder, NotificationOptions},
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
            apns_collapse_id: None,
        };

        let mut notification = a2::DefaultNotificationBuilder::new();

        if payload.is_encrypted() {
            notification = notification.set_content_available().set_mutable_content();
        } else {
            let blob: DecryptedPayloadBlob = serde_json::from_str(&payload.blob)?;
            notification = notification.set_title(&blob.title).set_body(&blob.body);
        }

        let mut notification_payload = notification.build(token.as_str(), opt);

        if payload.is_encrypted() {
            notification_payload.add_custom_data("encrypted_payload", &payload)?;
        }

        let _ = self.client.send(notification_payload).await?;

        Ok(())
    }
}
