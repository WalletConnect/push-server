use crate::error::Result;
use crate::handlers::push_message::PushMessageBody;
use crate::handlers::register_client::RegisterBody;
use crate::handlers::Response;
use crate::middleware::validate_signature::RequireValidSignature;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

pub async fn delete_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
) -> Result<Response> {
    crate::handlers::delete_client::handler(
        Path((state.config.default_tenant_id.clone(), id)),
        state,
    )
    .await
}

pub async fn push_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    valid_sig: RequireValidSignature<Json<PushMessageBody>>,
) -> Result<Response> {
    crate::handlers::push_message::handler(
        Path((state.config.default_tenant_id.clone(), id)),
        state,
        valid_sig,
    )
    .await
}

pub async fn register_handler(
    state: State<Arc<AppState>>,
    body: Json<RegisterBody>,
) -> Result<Response> {
    crate::handlers::register_client::handler(
        Path(state.config.default_tenant_id.clone()),
        state,
        body,
    )
    .await
}
