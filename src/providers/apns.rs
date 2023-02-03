use {
    crate::{
        blob::DecryptedPayloadBlob,
        handlers::push_message::MessagePayload,
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

        // TODO tidy after https://github.com/WalletConnect/a2/issues/67 is closed
        if payload.is_encrypted() {
            let mut notification_payload = a2::DefaultNotificationBuilder::new()
                .set_content_available()
                .set_mutable_content()
                .set_title("much <3 love")
                .build(token.as_str(), opt);

            notification_payload.add_custom_data("topic", &payload.topic)?;
            notification_payload.add_custom_data("blob", &payload.blob)?;

            let _ = self.client.send(notification_payload).await?;
        } else {
            let blob = DecryptedPayloadBlob::from_base64_encoded(payload.blob)?;

            let notification_payload = a2::DefaultNotificationBuilder::new()
                .set_title(&blob.title)
                .set_body(&blob.body)
                .build(token.as_str(), opt);

            let _ = self.client.send(notification_payload).await?;
        }

        Ok(())
    }
}
