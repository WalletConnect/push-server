use {
    crate::{error::Error, state::AppState, stores::tenant::TenantUpdateParams},
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

    let params = TenantUpdateParams {
        id: Some(body.id),

        fcm_api_key: None,

        apns_topic: None,
        apns_certificate: None,
        apns_certificate_password: None,
    };

    let tenant = state.tenant_store.create_tenant(params).await?;

    Ok(Json(TenantRegisterResponse {
        url: format!("{}/{}", state.config.public_url, tenant.id),
    }))
}
