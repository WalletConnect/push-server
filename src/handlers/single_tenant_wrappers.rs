use {
    crate::{
        error::{Error::MissingTenantId, Result},
        handlers::{push_message::PushMessageBody, register_client::RegisterBody, Response},
        middleware::validate_signature::RequireValidSignature,
        state::{AppState, State},
    },
    axum::{
        extract::{Path, State as StateExtractor},
        Json,
    },
    std::sync::Arc,
};

pub async fn delete_handler(
    Path(id): Path<String>,
    state: StateExtractor<Arc<AppState>>,
) -> Result<Response> {
    if state.is_multitenant() {
        return Err(MissingTenantId);
    }

    crate::handlers::delete_client::handler(
        Path((state.config.default_tenant_id.clone(), id)),
        state,
    )
    .await
}

pub async fn push_handler(
    Path(id): Path<String>,
    state: StateExtractor<Arc<AppState>>,
    valid_sig: RequireValidSignature<Json<PushMessageBody>>,
) -> Result<Response> {
    if state.is_multitenant() {
        return Err(MissingTenantId);
    }

    crate::handlers::push_message::handler(
        Path((state.config.default_tenant_id.clone(), id)),
        state,
        valid_sig,
    )
    .await
}

pub async fn register_handler(
    state: StateExtractor<Arc<AppState>>,
    body: Json<RegisterBody>,
) -> Result<Response> {
    if state.is_multitenant() {
        return Err(MissingTenantId);
    }

    crate::handlers::register_client::handler(
        Path(state.config.default_tenant_id.clone()),
        state,
        body,
    )
    .await
}
