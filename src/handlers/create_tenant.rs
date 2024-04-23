use {
    crate::{
        error::Error, handlers::validate_tenant_request, increment_counter, log::prelude::*,
        state::AppState, stores::tenant::TenantUpdateParams,
    },
    axum::{extract::State, http::HeaderMap, Json},
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    tracing::instrument,
};

#[derive(Serialize, Deserialize)]
pub struct TenantRegisterBody {
    /// The project ID
    pub id: String,
}

#[derive(Serialize)]
pub struct TenantRegisterResponse {
    /// The generated tenant url for the specified project id
    pub url: String,
}

#[instrument(skip_all, name = "create_tenant_handler")]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<TenantRegisterBody>,
) -> Result<Json<TenantRegisterResponse>, Error> {
    #[cfg(feature = "cloud")]
    if let Err(e) = validate_tenant_request(&state.jwt_validation_client, &headers, &body.id).await
    {
        error!(
            tenant_id = %body.id,
            err = ?e,
            "JWT verification failed"
        );
        return Err(e);
    }

    #[cfg(not(feature = "cloud"))]
    if let Err(e) = validate_tenant_request(&state.jwt_validation_client, &headers) {
        error!(
            tenant_id = %body.id,
            err = ?e,
            "JWT verification failed"
        );
        return Err(e);
    }

    let params = TenantUpdateParams { id: body.id };

    let tenant = state.tenant_store.create_tenant(params).await?;

    increment_counter!(state.metrics, registered_tenants);

    debug!(
        tenant_id = %tenant.id,
        "new tenant"
    );

    Ok(Json(TenantRegisterResponse {
        url: format!("{}/{}", state.config.public_url, tenant.id),
    }))
}
