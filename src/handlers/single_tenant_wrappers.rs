use {
    crate::{
        error::Result,
        handlers::{push_message::PushMessageBody, register_client::RegisterBody, Response},
        middleware::validate_signature::RequireValidSignature,
        state::AppState,
        stores::tenant::DEFAULT_TENANT_ID,
    },
    axum::{
        extract::{Path, State as StateExtractor},
        Json,
    },
    hyper::HeaderMap,
    std::sync::Arc,
};
#[cfg(analytics)]
use {axum::ConnectInfo, std::net::SocketAddr};

#[cfg(multitenant)]
use crate::error::Error::MissingTenantId;

pub async fn delete_handler(
    Path(id): Path<String>,
    state: StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response> {
    #[cfg(multitenant)]
    return Err(MissingTenantId);

    #[cfg(all(not(multitenant)))]
    crate::handlers::delete_client::handler(
        Path((DEFAULT_TENANT_ID.to_string(), id)),
        state,
        headers,
    )
    .await
}

pub async fn push_handler(
    #[cfg(analytics)] addr: ConnectInfo<SocketAddr>,
    Path(id): Path<String>,
    state: StateExtractor<Arc<AppState>>,
    valid_sig: RequireValidSignature<Json<PushMessageBody>>,
) -> Result<Response> {
    #[cfg(multitenant)]
    return Err(MissingTenantId);

    #[cfg(all(not(multitenant), analytics))]
    return crate::handlers::push_message::handler(
        addr,
        Path((DEFAULT_TENANT_ID.to_string(), id)),
        state,
        valid_sig,
    )
    .await;

    #[cfg(all(not(multitenant), not(analytics)))]
    return crate::handlers::push_message::handler(
        Path((DEFAULT_TENANT_ID.to_string(), id)),
        state,
        valid_sig,
    )
    .await;
}

pub async fn register_handler(
    #[cfg(analytics)] addr: ConnectInfo<SocketAddr>,
    state: StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
    body: Json<RegisterBody>,
) -> Result<Response> {
    #[cfg(multitenant)]
    return Err(MissingTenantId);

    #[cfg(all(not(multitenant), analytics))]
    return crate::handlers::register_client::handler(
        addr,
        Path(DEFAULT_TENANT_ID.to_string()),
        state,
        headers,
        body,
    )
    .await;

    #[cfg(all(not(multitenant), not(analytics)))]
    return crate::handlers::register_client::handler(
        Path(DEFAULT_TENANT_ID.to_string()),
        state,
        headers,
        body,
    )
    .await;
}
