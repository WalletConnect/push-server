#[cfg(feature = "analytics")]
use axum_client_ip::SecureClientIp;
use {
    crate::{
        analytics::message_info::MessageInfo,
        blob::ENCRYPTED_FLAG,
        error::{
            Error,
            Error::{ClientNotFound, Store},
        },
        handlers::DECENTRALIZED_IDENTIFIER_PREFIX,
        increment_counter,
        log::prelude::*,
        middleware::validate_signature::RequireValidSignature,
        providers::{Provider, PushProvider},
        request_id::get_req_id,
        state::AppState,
        stores::StoreError,
    },
    axum::{
        extract::{Json, Path, State as StateExtractor},
        http::{HeaderMap, StatusCode},
        response::IntoResponse,
    },
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    tap::TapFallible,
    tracing::instrument,
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MessagePayload {
    pub topic: String,
    pub flags: u32,
    pub blob: String,
}

impl MessagePayload {
    pub fn is_encrypted(&self) -> bool {
        (self.flags & ENCRYPTED_FLAG) == ENCRYPTED_FLAG
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PushMessageBody {
    pub id: String,
    pub payload: MessagePayload,
}

pub async fn handler(
    #[cfg(feature = "analytics")] SecureClientIp(client_ip): SecureClientIp,
    Path((tenant_id, client_id)): Path<(String, String)>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
    RequireValidSignature(Json(body)): RequireValidSignature<Json<PushMessageBody>>,
) -> Result<axum::response::Response, Error> {
    let res = handler_internal(
        Path((tenant_id.clone(), client_id.clone())),
        StateExtractor(state.clone()),
        headers.clone(),
        RequireValidSignature(Json(body.clone())),
    )
    .await;

    #[cfg(feature = "analytics")]
    let request_id = get_req_id(&headers);

    let inner_packed = match res {
        Ok((res, analytics_options_inner)) => (res.status().as_u16(), res, analytics_options_inner),
        Err((error, analytics_option_inner)) => {
            warn!("error handling push message: {error:?}");

            #[cfg(feature = "analytics")]
            let error_str = format!("{:?}", &error);
            let res = error.into_response();
            let status_code = res.status().clone().as_u16();

            let mut analytics_option = None;
            if let Some(analytics_unwrapped) = analytics_option_inner {
                #[cfg(feature = "analytics")]
                {
                    analytics_option = Some(MessageInfo {
                        response_message: Some(error_str.into()),
                        ..analytics_unwrapped
                    });
                }

                #[cfg(not(feature = "analytics"))]
                {
                    analytics_option = Some(analytics_unwrapped);
                }
            }

            (status_code, res, analytics_option)
        }
    };

    #[cfg(feature = "analytics")]
    let (status, response, analytics_option) = inner_packed;

    #[cfg(not(feature = "analytics"))]
    let (_status, response, _analytics_option) = inner_packed;

    #[cfg(feature = "analytics")]
    if let Some(mut message_info) = analytics_option {
        message_info.status = status;

        tokio::spawn(async move {
            if let Some(analytics) = &state.analytics {
                let (country, continent, region) = analytics
                    .lookup_geo_data(client_ip)
                    .map_or((None, None, None), |geo| {
                        (geo.country, geo.continent, geo.region)
                    });

                debug!(
                    %request_id,
                    %tenant_id,
                    client_id = %client_id,
                    ip = %client_ip,
                    "loaded geo data"
                );

                message_info.country = country;
                message_info.continent = continent;
                message_info.region = region.map(|r| Arc::from(r.join(", ")));

                analytics.message(message_info);
            }
        });
    }

    Ok(response)
}

#[instrument(name = "push_message_internal", skip_all, fields(tenant_id = tenant_id, client_id = client_id, id = body.id))]
pub async fn handler_internal(
    Path((tenant_id, client_id)): Path<(String, String)>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
    RequireValidSignature(Json(body)): RequireValidSignature<Json<PushMessageBody>>,
) -> Result<(axum::response::Response, Option<MessageInfo>), (Error, Option<MessageInfo>)> {
    #[cfg(feature = "analytics")]
    let topic: Arc<str> = body.payload.clone().topic.into();

    #[cfg(feature = "analytics")]
    let (flags, encrypted) = (body.payload.clone().flags, body.payload.is_encrypted());

    let client = match state.client_store.get_client(&tenant_id, &client_id).await {
        Ok(c) => Ok(c),
        Err(StoreError::NotFound(_, _)) => Err(ClientNotFound),
        Err(e) => Err(Store(e)),
    }
    .map_err(|e| {
        (
            e,
            #[cfg(feature = "analytics")]
            Some(MessageInfo {
                msg_id: body.id.clone().into(),
                region: None,
                country: None,
                continent: None,
                project_id: tenant_id.clone().into(),
                client_id: client_id.clone().into(),
                topic: topic.clone(),
                push_provider: "unknown".into(),
                encrypted,
                flags,
                status: 0,
                response_message: None,
                received_at: wc::analytics::time::now(),
            }),
            #[cfg(not(feature = "analytics"))]
            None,
        )
    })?;

    #[cfg(feature = "analytics")]
    let mut analytics = Some(MessageInfo {
        msg_id: body.id.clone().into(),
        region: None,
        country: None,
        continent: None,
        project_id: tenant_id.clone().into(),
        client_id: client_id.clone().into(),
        topic,
        push_provider: client.push_type.as_str().into(),
        encrypted,
        flags,
        status: 0,
        response_message: None,
        received_at: wc::analytics::time::now(),
    });

    #[cfg(not(feature = "analytics"))]
    let analytics = None;

    let request_id = get_req_id(&headers);

    increment_counter!(state.metrics, received_notifications);

    let client_id = client_id
        .trim_start_matches(DECENTRALIZED_IDENTIFIER_PREFIX)
        .to_string();

    debug!(
        %request_id,
        %tenant_id,
        client_id = %client_id,
        "fetched client to send notification"
    );

    if tenant_id != client.tenant_id {
        warn!(
            %request_id,
            %tenant_id,
            client_id = %client_id,
            "client tenant id does not match request tenant id"
        );

        #[cfg(feature = "multitenant")]
        {
            if client.tenant_id == "0000-0000-0000-0000" {
                warn!(
                    %request_id,
                    %tenant_id,
                    client_id = %client_id,
                    "client tenant id has not been set, allowing request to continue"
                );
            } else {
                #[cfg(feature = "analytics")]
                {
                    analytics = Some(MessageInfo {
                        response_message: Some(
                            "Client tenant id does not match request tenant id".into(),
                        ),
                        ..analytics.unwrap()
                    });

                    return Err((Error::MissmatchedTenantId, analytics));
                }

                #[cfg(not(feature = "analytics"))]
                return Err((Error::MissmatchedTenantId, None));
            }
        }

        #[cfg(not(feature = "multitenant"))]
        {
            #[cfg(feature = "analytics")]
            {
                analytics = Some(MessageInfo {
                    response_message: Some(
                        "Client tenant id does not match request tenant id".into(),
                    ),
                    ..analytics.unwrap()
                });

                return Err((Error::MissmatchedTenantId, analytics));
            }

            #[cfg(not(feature = "analytics"))]
            return Err((Error::MissmatchedTenantId, None));
        }
    }

    if let Ok(notification) = state
        .notification_store
        .get_notification(&body.id, &client_id, &tenant_id)
        .await
    {
        warn!(
            %request_id,
            %tenant_id,
            client_id = %client_id,
            notification_id = %notification.id,
            last_recieved_at = %notification.last_received_at,
            "notification has already been received"
        );

        #[cfg(feature = "analytics")]
        {
            analytics = Some(MessageInfo {
                response_message: Some("Notification has already been received".into()),
                ..analytics.unwrap()
            });

            return Ok(((StatusCode::OK).into_response(), analytics));
        }

        #[cfg(not(feature = "analytics"))]
        return Ok(((StatusCode::OK).into_response(), None));
    }

    let notification = state
        .notification_store
        .create_or_update_notification(&body.id, &tenant_id, &client_id, &body.payload)
        .await
        .tap_err(|e| warn!("error create_or_update_notification: {e:?}"))
        .map_err(|e| (Error::Store(e), analytics.clone()))?;

    info!(
        %request_id,
        %tenant_id,
        client_id = %client_id,
        notification_id = %notification.id,
        "stored notification",
    );

    // TODO make better by only ignoring if previously executed successfully
    // If notification received more than once then discard
    if notification.previous_payloads.len() > 1 {
        warn!(
            %request_id,
            %tenant_id,
            client_id = %client_id,
            notification_id = %notification.id,
            last_recieved_at = %notification.last_received_at,
            "notification has already been processed"
        );

        #[cfg(feature = "analytics")]
        {
            analytics = Some(MessageInfo {
                response_message: Some("Notification has already been processed".into()),
                ..analytics.unwrap()
            });

            return Ok(((StatusCode::OK).into_response(), analytics));
        }

        #[cfg(not(feature = "analytics"))]
        return Ok(((StatusCode::OK).into_response(), None));
    }

    let tenant = state
        .tenant_store
        .get_tenant(&tenant_id)
        .await
        .tap_err(|e| warn!("error fetching tenant: {e:?}"))
        .map_err(|e| (e, analytics.clone()))?;
    debug!(
        %request_id,
        %tenant_id,
        client_id = %client_id,
        notification_id = %notification.id,
        "fetched tenant"
    );

    if tenant.suspended {
        warn!("tenant suspended");
        return Err((Error::TenantSuspended, analytics.clone()));
    }

    let mut provider = tenant
        .provider(&client.push_type)
        .tap_err(|e| warn!("error fetching provider: {e:?}"))
        .map_err(|e| (e, analytics.clone()))?;
    debug!(
        %request_id,
        %tenant_id,
        client_id = %client_id,
        notification_id = %notification.id,
        push_type = client.push_type.as_str(),
        "fetched provider"
    );

    match provider.send_notification(client.token, body.payload).await {
        Ok(()) => Ok(()),
        Err(error) => {
            warn!("error sending notification: {error:?}");
            match error {
                Error::BadDeviceToken(_) => {
                    state
                        .client_store
                        .delete_client(&tenant_id, &client_id)
                        .await
                        .map_err(|e| (Error::Store(e), analytics.clone()))?;
                    increment_counter!(state.metrics, client_suspensions);
                    warn!(
                        %request_id,
                        %tenant_id,
                        client_id = %client_id,
                        notification_id = %notification.id,
                        push_type = client.push_type.as_str(),
                        "client has been deleted due to a bad device token"
                    );
                    Err(Error::ClientDeleted)
                }
                Error::BadApnsCredentials => {
                    state
                        .tenant_store
                        .suspend_tenant(&tenant_id, "Invalid APNS Credentials")
                        .await
                        .map_err(|e| (e, analytics.clone()))?;
                    increment_counter!(state.metrics, tenant_suspensions);
                    warn!(
                        %request_id,
                        %tenant_id,
                        client_id = %client_id,
                        notification_id = %notification.id,
                        push_type = client.push_type.as_str(),
                        "tenant has been suspended due to invalid provider credentials"
                    );
                    Err(Error::TenantSuspended)
                }
                Error::BadFcmApiKey => {
                    state
                        .tenant_store
                        .suspend_tenant(&tenant_id, "Invalid FCM Credentials")
                        .await
                        .map_err(|e| (e, analytics.clone()))?;
                    increment_counter!(state.metrics, tenant_suspensions);
                    warn!(
                        %request_id,
                        %tenant_id,
                        client_id = %client_id,
                        notification_id = %notification.id,
                        push_type = client.push_type.as_str(),
                        "tenant has been suspended due to invalid provider credentials"
                    );
                    Err(Error::TenantSuspended)
                }
                e => Err(e),
            }
        }
    }
    .map_err(|e| (e, analytics.clone()))?;

    info!(
        %request_id,
        %tenant_id,
        client_id = %client_id,
        notification_id = %notification.id,
        push_type = client.push_type.as_str(),
        "sent notification"
    );

    // Provider specific metrics
    match provider {
        Provider::Fcm(_) => increment_counter!(state.metrics, sent_fcm_notifications),
        Provider::Apns(_) => increment_counter!(state.metrics, sent_apns_notifications),
        #[cfg(any(debug_assertions, test))]
        Provider::Noop(_) => {}
    }

    #[cfg(feature = "analytics")]
    {
        analytics = Some(MessageInfo {
            response_message: Some("Delivered".into()),
            ..analytics.unwrap()
        });

        return Ok(((StatusCode::ACCEPTED).into_response(), analytics));
    }

    #[cfg(not(feature = "analytics"))]
    Ok(((StatusCode::ACCEPTED).into_response(), None))
}
