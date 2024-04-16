use {
    super::{LegacyPushMessage, PushMessage},
    crate::{blob::DecryptedPayloadBlob, error::Error, providers::PushProvider},
    async_trait::async_trait,
    fcm_v1::{
        AndroidConfig, AndroidMessagePriority, AndroidNotification, Client, Error as FcmError,
        Message, Notification, Target, WebpushConfig,
    },
    std::fmt::{Debug, Formatter},
    tracing::{debug, instrument},
};

pub struct FcmV1Provider {
    credentials: String,
    client: Client,
}

impl FcmV1Provider {
    pub fn new(credentials: String) -> Self {
        Self {
            credentials,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl PushProvider for FcmV1Provider {
    #[instrument(name = "send_fcm_v1_notification")]
    async fn send_notification(
        &mut self,
        token: String,
        body: PushMessage,
    ) -> crate::error::Result<()> {
        let result = match body {
            PushMessage::RawPushMessage(message) => {
                // Sending `always_raw` encrypted message
                debug!("Sending raw encrypted message");
                let message = Message {
                    data: Some(serde_json::to_value(message)?),
                    notification: None,
                    target: Target::Token(token),
                    android: Some(AndroidConfig {
                        priority: Some(AndroidMessagePriority::High),
                        collapse_key: None,
                        ttl: None,
                        restricted_package_name: None,
                        data: None,
                        notification: None,
                        fcm_options: None,
                        direct_boot_ok: None,
                    }),
                    // TODO
                    webpush: Some(WebpushConfig {
                        headers: None,
                        data: None,
                        notification: None,
                        fcm_options: None,
                    }),
                    // TODO do we need to set this for iOS React Native? We are missing content_available equivalent
                    apns: None,
                    fcm_options: None,
                };
                self.client.send(message).await
            }
            PushMessage::LegacyPushMessage(LegacyPushMessage { id: _, payload }) => {
                if payload.is_encrypted() {
                    debug!("Sending legacy `is_encrypted` message");
                    let message = Message {
                        data: Some(serde_json::to_value(payload)?),
                        notification: None,
                        target: Target::Token(token),
                        android: Some(AndroidConfig {
                            priority: Some(AndroidMessagePriority::High),
                            collapse_key: None,
                            ttl: None,
                            restricted_package_name: None,
                            data: None,
                            notification: None,
                            fcm_options: None,
                            direct_boot_ok: None,
                        }),
                        // TODO
                        webpush: Some(WebpushConfig {
                            headers: None,
                            data: None,
                            notification: None,
                            fcm_options: None,
                        }),
                        // TODO do we need to set this for iOS React Native? We are missing content_available equivalent
                        apns: None,
                        fcm_options: None,
                    };
                    self.client.send(message).await
                } else {
                    debug!("Sending plain message");
                    let blob = DecryptedPayloadBlob::from_base64_encoded(&payload.blob)?;

                    let message = Message {
                        data: Some(serde_json::to_value(payload.to_owned())?),
                        notification: Some(Notification {
                            title: Some(blob.title.clone()),
                            body: Some(blob.body.clone()),
                            image: None,
                        }),
                        target: Target::Token(token),
                        android: Some(AndroidConfig {
                            priority: Some(AndroidMessagePriority::High),
                            collapse_key: None,
                            ttl: None,
                            restricted_package_name: None,
                            data: None,
                            notification: Some(AndroidNotification {
                                notification_priority: None,
                                // TODO do we need to override this of already set in `notification` above?
                                title: Some(blob.title),
                                body: Some(blob.body),
                                icon: None,
                                color: None,
                                sound: None,
                                tag: None,
                                click_action: None,
                                body_loc_key: None,
                                body_loc_args: None,
                                title_loc_key: None,
                                title_loc_args: None,
                                channel_id: None,
                                ticker: None,
                                sticky: None,
                                event_time: None,
                                local_only: None,
                                default_sound: None,
                                default_vibrate_timings: None,
                                default_light_settings: None,
                                vibrate_timings: None,
                                visibility: None,
                                notification_count: None,
                                light_settings: None,
                                image: None,
                            }),
                            fcm_options: None,
                            direct_boot_ok: None,
                        }),
                        // TODO
                        webpush: Some(WebpushConfig {
                            headers: None,
                            data: None,
                            notification: None,
                            fcm_options: None,
                        }),
                        // TODO do we need to set this for iOS React Native? We are missing content_available equivalent
                        apns: None,
                        fcm_options: None,
                    };
                    self.client.send(message).await
                }
            }
        };

        match result {
            Ok(_val) => {
                // FIXME
                // let FcmResponse { error, .. } = val;
                // if let Some(error) = error {
                //     match error {
                //         ErrorReason::MissingRegistration => Err(Error::BadDeviceToken(
                //             "Missing registration for token".into(),
                //         )),
                //         ErrorReason::InvalidRegistration => {
                //             Err(Error::BadDeviceToken("Invalid token registration".into()))
                //         }
                //         ErrorReason::NotRegistered => {
                //             Err(Error::BadDeviceToken("Token is not registered".into()))
                //         }
                //         ErrorReason::InvalidApnsCredential => Err(Error::BadApnsCredentials),
                //         e => Err(Error::FcmResponse(e)),
                //     }
                // } else {
                Ok(())
                // }
            }
            Err(e) => match e {
                FcmError::Unauthorized => Err(Error::BadFcmApiKey),
                e => Err(Error::FcmV1(e)),
            },
        }
    }
}

// Manual Impl Because `fcm::Client` does not derive anything and doesn't need
// to be accounted for

impl Clone for FcmV1Provider {
    fn clone(&self) -> Self {
        FcmV1Provider {
            credentials: self.credentials.clone(),
            client: Client::new(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.credentials = source.credentials.clone();
        self.client = Client::new();
    }
}

impl PartialEq for FcmV1Provider {
    fn eq(&self, other: &Self) -> bool {
        self.credentials == other.credentials
    }
}

impl Debug for FcmV1Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[FcmV1Provider] api_key = {}", self.credentials)
    }
}
