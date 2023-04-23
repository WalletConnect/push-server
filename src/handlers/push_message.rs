#[cfg(feature = "analytics")]
use {
    crate::analytics::message_info::MessageInfo,
    axum::extract::ConnectInfo,
    std::net::SocketAddr,
};
use {
    crate::{
        blob::ENCRYPTED_FLAG,
        error::{
            Error::{ClientNotFound, Store},
            Result,
        },
        handlers::{Response, DECENTRALIZED_IDENTIFIER_PREFIX},
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
    },
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MessagePayload {
    pub topic: Option<String>,
    pub flags: u32,
    pub blob: String,
}

impl MessagePayload {
    pub fn is_encrypted(&self) -> bool {
        (self.flags & ENCRYPTED_FLAG) == ENCRYPTED_FLAG
    }
}

#[derive(Serialize, Deserialize)]
pub struct PushMessageBody {
    pub id: String,
    pub payload: MessagePayload,
}

pub async fn handler(
    #[cfg(feature = "analytics")] ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path((tenant_id, id)): Path<(String, String)>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
    RequireValidSignature(Json(body)): RequireValidSignature<Json<PushMessageBody>>,
) -> Result<Response> {
    let request_id = get_req_id(&headers);

    increment_counter!(state.metrics, received_notifications);

    #[cfg(feature = "analytics")]
    let (flags, encrypted) = (body.payload.flags, body.payload.is_encrypted());

    let id = id
        .trim_start_matches(DECENTRALIZED_IDENTIFIER_PREFIX)
        .to_string();

    let client = match state.client_store.get_client(&tenant_id, &id).await {
        Ok(c) => Ok(c),
        Err(StoreError::NotFound(_, _)) => Err(ClientNotFound),
        Err(e) => Err(Store(e)),
    }?;

    debug!(
        %request_id,
        %tenant_id,
        client_id = %id,
        "fetched client to send notification"
    );

    if let Ok(notification) = state
        .notification_store
        .get_notification(&body.id, &tenant_id)
        .await
    {
        warn!(
            %request_id,
            %tenant_id,
            client_id = %id,
            notification_id = %notification.id,
            last_recieved_at = %notification.last_received_at,
            "notification has already been received"
        );
        return Ok(Response::new_success(StatusCode::OK));
    }

    let notification = state
        .notification_store
        .create_or_update_notification(&body.id, &tenant_id, &id, &body.payload)
        .await?;
    info!(
        %request_id,
        %tenant_id,
        client_id = %id,
        notification_id = %notification.id,
        "stored notification",
    );

    // TODO make better by only ignoring if previously executed successfully
    // If notification received more than once then discard
    if notification.previous_payloads.len() > 1 {
        warn!(
            %request_id,
            %tenant_id,
            client_id = %id,
            notification_id = %notification.id,
            last_recieved_at = %notification.last_received_at,
            "notification has already been processed"
        );
        return Ok(Response::new_success(StatusCode::OK));
    }

    let tenant = state.tenant_store.get_tenant(&tenant_id).await?;
    debug!(
        %request_id,
        %tenant_id,
        client_id = %id,
        notification_id = %notification.id,
        "fetched tenant"
    );

    let mut provider = tenant.provider(&client.push_type)?;
    debug!(
        %request_id,
        %tenant_id,
        client_id = %id,
        notification_id = %notification.id,
        push_type = client.push_type.as_str(),
        "fetched provider"
    );

    #[cfg(feature = "analytics")]
    let topic: Option<Arc<str>> = body.payload.topic.as_ref().map(|t| t.clone().into());

    provider
        .send_notification(client.token, body.payload)
        .await?;

    info!(
        %request_id,
        %tenant_id,
        client_id = %id,
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

    // Analytics
    #[cfg(feature = "analytics")]
    tokio::spawn(async move {
        if let Some(analytics) = &state.analytics {
            let (country, continent, region) = analytics
                .geoip
                .lookup_geo_data(addr.ip())
                .map_or((None, None, None), |geo| {
                    (geo.country, geo.continent, geo.region)
                });

            debug!(
                %request_id,
                %tenant_id,
                %client_id,
                ip = %addr.ip(),
                "loaded geo data"
            );

            let msg = MessageInfo {
                region: region.map(|r| Arc::from(r.join(", "))),
                country,
                continent,
                project_id: tenant_id.into(),
                client_id: id.into(),
                topic,
                push_provider: client.push_type.as_str().into(),
                encrypted,
                flags,
                received_at: gorgon::time::now(),
            };

            analytics.message(msg);
        }
    });

    Ok(Response::new_success(StatusCode::ACCEPTED))
}
