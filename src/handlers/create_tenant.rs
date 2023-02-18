use {
    crate::{error::Error, increment_counter, state::AppState, stores::tenant::TenantUpdateParams},
    axum::{extract::State, Json},
    serde::{Deserialize, Serialize},
    std::sync::Arc,
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
    Json(body): Json<TenantRegisterBody>,
) -> Result<Json<TenantRegisterResponse>, Error> {
    // TODO authentication
    // TODO validation

    let params = TenantUpdateParams { id: body.id };

    let tenant = state.tenant_store.create_tenant(params).await?;

    increment_counter!(state.metrics, registered_tenants);

    Ok(Json(TenantRegisterResponse {
        url: format!("{}/{}", state.config.public_url, tenant.id),
    }))
}
