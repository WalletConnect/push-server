use {
    crate::{
        error::{Error::MissingTenantId, Result},
        handlers::{push_message::PushMessageBody, register_client::RegisterBody, Response},
        middleware::validate_signature::RequireValidSignature,
        state::{AppState, State},
    },
    axum::{
        extract::{ConnectInfo, Path, State as StateExtractor},
        Json,
    },
    hyper::HeaderMap,
    std::{net::SocketAddr, sync::Arc},
};

pub async fn delete_handler(
    Path(id): Path<String>,
    state: StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response> {
    if state.is_multitenant() {
        return Err(MissingTenantId);
    }

    crate::handlers::delete_client::handler(
        Path((state.config.default_tenant_id.clone(), id)),
        state,
        headers,
    )
    .await
}

pub async fn push_handler(
    addr: ConnectInfo<SocketAddr>,
    Path(id): Path<String>,
    state: StateExtractor<Arc<AppState>>,
    valid_sig: RequireValidSignature<Json<PushMessageBody>>,
) -> Result<Response> {
    if state.is_multitenant() {
        return Err(MissingTenantId);
    }

    crate::handlers::push_message::handler(
        addr,
        Path((state.config.default_tenant_id.clone(), id)),
        state,
        valid_sig,
    )
    .await
}

pub async fn register_handler(
    addr: ConnectInfo<SocketAddr>,
    state: StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
    body: Json<RegisterBody>,
) -> Result<Response> {
    if state.is_multitenant() {
        return Err(MissingTenantId);
    }

    crate::handlers::register_client::handler(
        addr,
        Path(state.config.default_tenant_id.clone()),
        state,
        headers,
        body,
    )
    .await
}
