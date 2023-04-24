use {
    crate::{
        error::Error,
        handlers::validate_tenant_request,
        log::prelude::*,
        providers::ProviderKind,
        request_id::get_req_id,
        state::AppState,
        stores::tenant::ApnsType,
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
pub struct GetTenantResponse {
    url: String,
    enabled_providers: Vec<String>,
    apns_topic: Option<String>,
    apns_type: Option<ApnsType>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<GetTenantResponse>, Error> {
    let request_id = get_req_id(&headers);

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

    info!(
        %request_id,
        tenant_id = %id,
        "requested tenant"
    );

    Ok(Json(res))
}
