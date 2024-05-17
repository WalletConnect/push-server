use {
    crate::{error::Error, handlers::validate_tenant_request, log::prelude::*, state::AppState},
    axum::{
        extract::{Path, State},
        http::HeaderMap,
        Json,
    },
    serde::Serialize,
    std::sync::Arc,
    tracing::instrument,
};

#[derive(Serialize)]
pub struct DeleteTenantResponse {
    success: bool,
}

#[instrument(skip_all, name = "delete_tenant_handler")]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<DeleteTenantResponse>, Error> {
    #[cfg(feature = "cloud")]
    let verification_res =
        validate_tenant_request(&state.jwt_validation_client, &headers, &id).await;

    #[cfg(not(feature = "cloud"))]
    let verification_res = validate_tenant_request(&state.jwt_validation_client, &headers);

    if let Err(e) = verification_res {
        error!(
            tenant_id = %id,
            err = ?e,
            "JWT verification failed"
        );
        return Err(e);
    }

    state.tenant_store.delete_tenant(&id).await?;

    debug!(
        tenant_id = %id,
        "deleted tenant"
    );

    Ok(Json(DeleteTenantResponse { success: true }))
}
