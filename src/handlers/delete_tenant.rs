use {
    crate::{
        decrement_counter,
        error::{Error, Error::InvalidAuthentication},
        handlers::validate_tenant_request,
        log::prelude::*,
        request_id::get_req_id,
        state::AppState,
    },
    axum::{
        extract::{Path, State},
        http::{header::AUTHORIZATION, HeaderMap},
        Json,
    },
    cerberus::registry::RegistryClient,
    serde::Serialize,
    std::sync::Arc,
};

#[derive(Serialize)]
pub struct DeleteTenantResponse {
    success: bool,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<DeleteTenantResponse>, Error> {
    let req_id = get_req_id(&headers);

    validate_tenant_request(&state.registry_client, &state.gotrue_client, &headers, None)?;

    state.tenant_store.delete_tenant(&id).await?;

    decrement_counter!(state.metrics, registered_tenants);

    info!(
        request_id = %req_id,
        tenant_id = %id,
        "deleted tenant"
    );

    Ok(Json(DeleteTenantResponse { success: true }))
}
