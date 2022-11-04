use crate::error::Result;
use crate::state::AppState;
use crate::{handlers::Response, providers::PushProvider};
use crate::{middleware::validate_signature::RequireValidSignature, providers::get_provider};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    Path((tenant, id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    RequireValidSignature(Json(body)): RequireValidSignature<Json<PushMessageBody>>,
) -> Result<Response> {
    let client = state.client_store.get_client(&id).await?;

    let notification = state
        .notification_store
        .create_or_update_notification(&body.id, &tenant, &id, &body.payload)
        .await?;

    // TODO make better by only ignoring if previously executed successfully
    // If notification received more than once then discard
    if notification.previous_payloads.len() > 1 {
        return Ok(Response::new_success(StatusCode::ACCEPTED));
    }

    let mut provider = state
        .tenant_store
        .get_tenant_provider(&tenant, &client.push_type)
        .await?;

    provider
        .send_notification(client.token, body.payload)
        .await?;

    Ok(Response::new_success(StatusCode::ACCEPTED))
}
