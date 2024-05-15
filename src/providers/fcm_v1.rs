use {
    super::{LegacyPushMessage, PushMessage},
    crate::{blob::DecryptedPayloadBlob, error::Error, providers::PushProvider},
    async_trait::async_trait,
    fcm_v1::{
        gauth::serv_account::ServiceAccountKey, AndroidConfig, AndroidMessagePriority, ApnsConfig,
        Client, ClientBuildError, ErrorReason, Message, Notification, Response, Target,
    },
    serde::Serialize,
    serde_json::json,
    std::sync::Arc,
    tracing::{debug, instrument},
};

#[derive(Debug, Clone)]
pub struct FcmV1Provider {
    client: Client,
}

impl FcmV1Provider {
    pub async fn new(
        credentials: ServiceAccountKey,
        http_client: reqwest::Client,
    ) -> Result<Self, ClientBuildError> {
        let client = Client::builder()
            .http_client(http_client)
            .build(credentials)
            .await?;
        Ok(Self { client })
    }
}

#[async_trait]
impl PushProvider for FcmV1Provider {
    #[instrument(name = "send_fcm_v1_notification", skip_all)]
    async fn send_notification(
        &self,
        token: String,
        body: PushMessage,
    ) -> crate::error::Result<()> {
        let result = match body {
            PushMessage::RawPushMessage(message) => {
                // Sending `always_raw` encrypted message
                debug!("Sending raw encrypted message");
                #[derive(Serialize)]
                pub struct FcmV1RawPushMessage {
                    pub topic: Arc<str>,
                    pub tag: String,
                    pub message: Arc<str>,
                }
                let message = Message {
                    data: Some(
                        serde_json::to_value(FcmV1RawPushMessage {
                            // All keys must be strings
                            topic: message.topic.clone(),
                            tag: message.tag.to_string(),
                            message: message.message.clone(),
                        })
                        .map_err(Error::InternalSerializationError)?,
                    ),
                    notification: None,
                    target: Target::Token(token),
                    android: Some(AndroidConfig {
                        priority: Some(AndroidMessagePriority::High),
                        ..Default::default()
                    }),
                    webpush: None,
                    apns: Some(ApnsConfig {
                        payload: Some(json!({
                            "aps": {
                                "content-available": 1,
                            }
                        })),
                        ..Default::default()
                    }),
                    fcm_options: None,
                };
                self.client.send(message).await
            }
            PushMessage::LegacyPushMessage(LegacyPushMessage { id: _, payload }) => {
                #[derive(Serialize)]
                pub struct FcmV1MessagePayload {
                    pub topic: Arc<str>,
                    pub flags: String,
                    pub blob: Arc<str>,
                }
                let data = serde_json::to_value(FcmV1MessagePayload {
                    // All keys must be strings
                    topic: payload.topic.clone(),
                    flags: payload.flags.to_string(),
                    blob: payload.blob.clone(),
                })
                .map_err(Error::InternalSerializationError)?;

                if payload.is_encrypted() {
                    debug!("Sending legacy `is_encrypted` message");
                    let message = Message {
                        data: Some(data),
                        notification: None,
                        target: Target::Token(token),
                        android: Some(AndroidConfig {
                            priority: Some(AndroidMessagePriority::High),
                            ..Default::default()
                        }),
                        webpush: None,
                        apns: Some(ApnsConfig {
                            payload: Some(json!({
                                "aps": {
                                    "content-available": 1,
                                }
                            })),
                            ..Default::default()
                        }),
                        fcm_options: None,
                    };
                    self.client.send(message).await
                } else {
                    debug!("Sending plain message");
                    let blob = DecryptedPayloadBlob::from_base64_encoded(&payload.blob)?;

                    let message = Message {
                        data: Some(data),
                        notification: Some(Notification {
                            title: Some(blob.title.clone()),
                            body: Some(blob.body.clone()),
                            ..Default::default()
                        }),
                        target: Target::Token(token),
                        android: None,
                        webpush: None,
                        apns: None,
                        fcm_options: None,
                    };
                    self.client.send(message).await
                }
            }
        };

        #[allow(clippy::match_single_binding)]
        match result {
            Ok(val) => {
                let Response { error, .. } = val;
                if let Some(error) = error {
                    match error {
                        ErrorReason::MissingRegistration => Err(Error::BadDeviceToken(
                            "Missing registration for token".into(),
                        )),
                        ErrorReason::InvalidRegistration => {
                            Err(Error::BadDeviceToken("Invalid token registration".into()))
                        }
                        ErrorReason::NotRegistered => {
                            Err(Error::BadDeviceToken("Token is not registered".into()))
                        }
                        ErrorReason::InvalidApnsCredential => Err(Error::BadApnsCredentials),
                        e => Err(Error::FcmV1Response(e)),
                    }
                } else {
                    Ok(())
                }
            }
            Err(e) => match e {
                // SendError::Unauthorized => Err(Error::BadFcmV1Credentials),
                e => Err(Error::FcmV1(e)),
            },
        }
    }
}
