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
    tracing::{span, warn},
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
                    .set_title("You have new notifications. Open to view")
                    .build(token.as_str(), opt);

                notification_payload.add_custom_data("topic", &payload.topic)?;
                notification_payload.add_custom_data("blob", &payload.blob)?;

                self.client.send(notification_payload).await
            }
            false => {
                let blob = DecryptedPayloadBlob::from_base64_encoded(payload.blob)?;

                let mut notification_payload = a2::DefaultNotificationBuilder::new()
                    .set_content_available()
                    .set_mutable_content()
                    .set_title(&blob.title)
                    .set_body(&blob.body)
                    .build(token.as_str(), opt);

                notification_payload.add_custom_data("topic", &payload.topic)?;

                self.client.send(notification_payload).await
            }
        };

        match result {
            Ok(response) => {
                if response.error.is_some() {
                    warn!(
                        "Unexpected APNS error. a2 lib shouldn't allow returning Ok containing \
                         error response. Status: {} Error: {:?}",
                        response.code, response.error
                    );
                    Err(Error::Apns(a2::Error::ResponseError(response)))
                } else {
                    Ok(())
                }
            }
            Err(e) => match e {
                a2::Error::ResponseError(res) => match res.error {
                    None => Err(Error::Apns(a2::Error::ResponseError(res))),
                    Some(response) => match response.reason {
                        ErrorReason::BadDeviceToken => {
                            Err(Error::BadDeviceToken("Bad device token".to_string()))
                        }
                        // Note: This will have the device deleted because the token was not for the
                        // configured topic
                        ErrorReason::DeviceTokenNotForTopic => Err(Error::BadDeviceToken(
                            "The device token does not match the specified topic".to_string(),
                        )),
                        ErrorReason::Unregistered => Err(Error::BadDeviceToken(
                            "The device token is inactive for the specified topic".to_string(),
                        )),
                        reason => Err(Error::ApnsResponse(reason)),
                    },
                },
                e => Err(Error::Apns(e)),
            },
        }
    }
}
