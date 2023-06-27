use {
    crate::{
        blob::DecryptedPayloadBlob,
        error::Error,
        handlers::push_message::MessagePayload,
        providers::PushProvider,
    },
    a2::{ErrorReason, NotificationBuilder, NotificationOptions},
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

    pub fn new_token<R>(
        pkcs8_pem: &mut R,
        key_id: String,
        team_id: String,
        endpoint: a2::Endpoint,
        topic: String,
    ) -> crate::error::Result<Self>
    where
        R: Read,
    {
        Ok(ApnsProvider {
            client: a2::Client::token(pkcs8_pem, key_id, team_id, endpoint)?,
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
        let result = match payload.is_encrypted() {
            true => {
                let mut notification_payload = a2::DefaultNotificationBuilder::new()
                    .set_content_available()
                    .set_mutable_content()
                    .set_title("much <3 love")
                    .build(token.as_str(), opt);

                notification_payload.add_custom_data("topic", &payload.topic)?;
                notification_payload.add_custom_data("blob", &payload.blob)?;

                self.client.send(notification_payload).await
            }
            false => {
                let blob = DecryptedPayloadBlob::from_base64_encoded(payload.blob)?;

                let notification_payload = a2::DefaultNotificationBuilder::new()
                    .set_title(&blob.title)
                    .set_body(&blob.body)
                    .build(token.as_str(), opt);

                self.client.send(notification_payload).await
            }
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => match e {
                a2::Error::ResponseError(res) => match res.error {
                    None => Err(Error::Apns(a2::Error::ResponseError(res))),
                    Some(response) => match response.reason {
                        ErrorReason::BadDeviceToken => Err(Error::BadDeviceToken),
                        reason => Err(Error::ApnsResponse(reason)),
                    },
                },
                e => Err(Error::Apns(e)),
            },
        }
    }
}
