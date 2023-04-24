use {
    crate::{
        error::Error,
        increment_counter,
        request_id::get_req_id,
        state::AppState,
        stores::tenant::TenantUpdateParams,
    },
    axum::{extract::State, http::HeaderMap, Json},
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    tracing::info,
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

pub async fn handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<TenantRegisterBody>,
) -> Result<Json<TenantRegisterResponse>, Error> {
    let req_id = get_req_id(&headers);

    // TODO authentication
    // TODO validation

    let params = TenantUpdateParams { id: body.id };

    let tenant = state.tenant_store.create_tenant(params).await?;

    increment_counter!(state.metrics, registered_tenants);

    info!(
        request_id = %req_id,
        tenant_id = %tenant.id,
        "new tenant"
    );

    Ok(Json(TenantRegisterResponse {
        url: format!("{}/{}", state.config.public_url, tenant.id),
    }))
}
