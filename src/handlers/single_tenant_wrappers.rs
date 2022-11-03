use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use crate::handlers::Response;
use crate::state::AppState;
use crate::stores::client::ClientStore;
use crate::stores::notification::NotificationStore;
use crate::error::Result;
use crate::handlers::push_message::PushMessageBody;
use crate::handlers::register_client::RegisterBody;
use crate::middleware::validate_signature::RequireValidSignature;
use crate::stores::tenant::TenantStore;

pub async fn delete_handler(
    id: Path<String>,
    state: State<Arc<AppState<impl ClientStore, impl NotificationStore, impl TenantStore>>>,
) -> Result<Response> {
    crate::handlers::delete_client::handler(Path(state.config.default_tenant_id.clone()), id, state).await
}

pub async fn push_handler(
    id: Path<String>,
    state: State<Arc<AppState<impl ClientStore, impl NotificationStore, impl TenantStore>>>,
    valid_sig: RequireValidSignature<Json<PushMessageBody>>,
) -> Result<Response> {
    crate::handlers::push_message::handler(Path(state.config.default_tenant_id.clone()), id, state, valid_sig).await
}

pub async fn register_handler(
    state: State<Arc<AppState<impl ClientStore, impl NotificationStore, impl TenantStore>>>,
    body: Json<RegisterBody>,
) -> Result<Response> {
    crate::handlers::register_client::handler(Path(state.config.default_tenant_id.clone()), state, body).await
}