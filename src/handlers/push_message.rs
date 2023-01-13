use {
    crate::{
        blob::ENCRYPTED_FLAG,
        error::Result,
        handlers::Response,
        log::prelude::*,
        middleware::validate_signature::RequireValidSignature,
        providers::PushProvider,
        state::AppState,
    },
    axum::{
        extract::{Json, Path, State as StateExtractor},
        http::StatusCode,
    },
    opentelemetry::Context,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
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

#[derive(Serialize, Deserialize)]
pub struct PushMessageBody {
    pub id: String,
    pub payload: MessagePayload,
}

pub async fn handler(
    Path((tenant_id, id)): Path<(String, String)>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    RequireValidSignature(Json(body)): RequireValidSignature<Json<PushMessageBody>>,
) -> Result<Response> {
    if let Some(metrics) = &state.metrics {
        metrics
            .received_notifications
            .add(&Context::current(), 1, &[]);
        debug!("incremented `received_notifications` counter")
    }

    let client = state.client_store.get_client(&tenant_id, &id).await?;
    debug!("fetched client ({}) for tenant ({})", &id, &tenant_id);

    let notification = state
        .notification_store
        .create_or_update_notification(&body.id, &tenant_id, &id, &body.payload)
        .await?;
    debug!(
        "stored notification ({}) for tenant ({})",
        &notification.id, &tenant_id
    );

    // TODO make better by only ignoring if previously executed successfully
    // If notification received more than once then discard
    if notification.previous_payloads.len() > 1 {
        info!(
            "notification ({}) already received for client ({})",
            body.id, id
        );
        return Ok(Response::new_success(StatusCode::ACCEPTED));
    }

    let tenant = state.tenant_store.get_tenant(&tenant_id).await?;
    debug!(
        "fetched tenant ({}) during notification ({})",
        &tenant_id, &notification.id
    );

    let mut provider = tenant.provider(&client.push_type)?;
    debug!(
        "fetched provider ({}) for tenant ({}) during notification ({})",
        client.push_type.as_str(),
        &tenant_id,
        &notification.id
    );

    provider
        .send_notification(client.token, body.payload)
        .await?;
    debug!(
        "sent notification to provider ({}) for tenant ({}) during notification ({})",
        client.push_type.as_str(),
        &tenant_id,
        &notification.id
    );

    if let Some(metrics) = &state.metrics {
        metrics.sent_notifications.add(&Context::current(), 1, &[]);
        debug!("incremented `sent_notifications` counter")
    }

    Ok(Response::new_success(StatusCode::ACCEPTED))
}
