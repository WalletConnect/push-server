use {
    crate::{error::Error, providers::ProviderKind, state::AppState, stores::tenant::ApnsType},
    axum::{
        extract::{Path, State},
        Json,
    },
    serde::Serialize,
    std::sync::Arc,
};

#[derive(Serialize)]
pub struct GetTenantResponse {
    url: String,
    enabled_providers: Vec<String>,
    apns_topic: Option<String>,
    apns_type: Option<ApnsType>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<GetTenantResponse>, Error> {
    let tenant = state.tenant_store.get_tenant(&id).await?;

    let providers = tenant.providers();

    let mut res = GetTenantResponse {
        url: format!("{}/{}", state.config.public_url, tenant.id),
        enabled_providers: tenant.providers().iter().map(Into::into).collect(),
        apns_topic: None,
        apns_type: None,
    };

    if providers.contains(&ProviderKind::Apns) {
        res.apns_topic = tenant.apns_topic;
        res.apns_type = tenant.apns_type;
    }

    Ok(Json(res))
}
