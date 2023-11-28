#[cfg(feature = "analytics")]
use axum_client_ip::SecureClientIp;
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

#[cfg(feature = "multitenant")]
use crate::error::Error::MissingTenantId;

pub async fn delete_handler(
    Path(id): Path<String>,
    state: StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response> {
    #[cfg(feature = "multitenant")]
    return Err(MissingTenantId);

    #[cfg(not(feature = "multitenant"))]
    crate::handlers::delete_client::handler(
        Path((DEFAULT_TENANT_ID.to_string(), id)),
        state,
        headers,
    )
    .await
}

pub async fn push_handler(
    #[cfg(feature = "analytics")] SecureClientIp(client_ip): SecureClientIp,
    Path(id): Path<String>,
    state: StateExtractor<Arc<AppState>>,
    valid_sig: RequireValidSignature<Json<PushMessageBody>>,
) -> Result<axum::response::Response> {
    #[cfg(feature = "multitenant")]
    return Err(MissingTenantId);

    #[cfg(all(not(feature = "multitenant"), feature = "analytics"))]
    return crate::handlers::push_message::handler(
        SecureClientIp(client_ip),
        Path((DEFAULT_TENANT_ID.to_string(), id)),
        state,
        headers,
        valid_sig,
    )
    .await;

    #[cfg(all(not(feature = "multitenant"), not(feature = "analytics")))]
    return crate::handlers::push_message::handler(
        Path((DEFAULT_TENANT_ID.to_string(), id)),
        state,
        valid_sig,
    )
    .await;
}

pub async fn register_handler(
    #[cfg(feature = "analytics")] SecureClientIp(client_ip): SecureClientIp,
    state: StateExtractor<Arc<AppState>>,
    headers: HeaderMap,
    body: Json<RegisterBody>,
) -> Result<Response> {
    #[cfg(feature = "multitenant")]
    return Err(MissingTenantId);

    #[cfg(all(not(feature = "multitenant"), feature = "analytics"))]
    return crate::handlers::register_client::handler(
        SecureClientIp(client_ip),
        Path(DEFAULT_TENANT_ID.to_string()),
        state,
        headers,
        body,
    )
    .await;

    #[cfg(all(not(feature = "multitenant"), not(feature = "analytics")))]
    return crate::handlers::register_client::handler(
        Path(DEFAULT_TENANT_ID.to_string()),
        state,
        headers,
        body,
    )
    .await;
}
