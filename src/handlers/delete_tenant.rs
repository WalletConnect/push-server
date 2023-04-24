use {
    crate::{
        decrement_counter,
        error::Error,
        handlers::validate_tenant_request,
        log::prelude::*,
        request_id::get_req_id,
        state::AppState,
    },
    axum::{
        extract::{Path, State},
        http::HeaderMap,
        Json,
    },
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

    #[cfg(feature = "cloud")]
    validate_tenant_request(
        &state.registry_client,
        &state.gotrue_client,
        &headers,
        id.clone(),
        None,
    )
    .await?;

    #[cfg(not(feature = "cloud"))]
    validate_tenant_request(&state.gotrue_client, &headers)?;

    state.tenant_store.delete_tenant(&id).await?;

    decrement_counter!(state.metrics, registered_tenants);

    info!(
        request_id = %req_id,
        tenant_id = %id,
        "deleted tenant"
    );

    Ok(Json(DeleteTenantResponse { success: true }))
}
