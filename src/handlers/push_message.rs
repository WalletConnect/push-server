use {
    crate::{
        error::{Error::IncludedTenantIdWhenNotNeeded, Result},
        handlers::Response,
        middleware::validate_signature::RequireValidSignature,
        providers::PushProvider,
        state::{AppState, State},
    },
    axum::{
        extract::{Json, Path, State as StateExtractor},
        http::StatusCode,
    },
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MessagePayload {
    pub title: String,
    pub description: String,
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
    if state.config.default_tenant_id != tenant_id && !state.is_multitenant() {
        return Err(IncludedTenantIdWhenNotNeeded);
    }

    let client = state.client_store.get_client(&tenant_id, &id).await?;

    let notification = state
        .notification_store
        .create_or_update_notification(&body.id, &tenant_id, &id, &body.payload)
        .await?;

    // TODO make better by only ignoring if previously executed successfully
    // If notification received more than once then discard
    if notification.previous_payloads.len() > 1 {
        return Ok(Response::new_success(StatusCode::ACCEPTED));
    }

    let tenant = state.tenant_store.get_tenant(&tenant_id).await?;

    let mut provider = tenant.provider(&client.push_type)?;

    provider
        .send_notification(client.token, body.payload)
        .await?;

    Ok(Response::new_success(StatusCode::ACCEPTED))
}
