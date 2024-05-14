use {
    crate::{
        error::Error,
        handlers::validate_tenant_request,
        log::prelude::*,
        providers::{ProviderKind, PROVIDER_FCM_V1},
        state::AppState,
        stores::tenant::ApnsType,
    },
    axum::{
        extract::{Path, State},
        http::HeaderMap,
        Json,
    },
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    tracing::instrument,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTenantResponse {
    pub url: String,
    pub enabled_providers: Vec<String>,
    pub apns_topic: Option<String>,
    pub apns_type: Option<ApnsType>,
    pub suspended: bool,
    pub suspended_reason: Option<String>,
}

#[instrument(skip_all, name = "get_tenant_handler")]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<GetTenantResponse>, Error> {
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

    let tenant = state.tenant_store.get_tenant(&id).await?;

    let providers = tenant.providers();

    let mut res = GetTenantResponse {
        url: format!("{}/{}", state.config.public_url, tenant.id),
        enabled_providers: tenant
            .providers()
            .iter()
            .map(Into::into)
            // Special case on fcm_v1 for credentials because providers() is also used for token management (of which FCM and FCM V1 tokens are the same)
            .chain(if tenant.fcm_v1_credentials.is_some() {
                vec![PROVIDER_FCM_V1.to_string()]
            } else {
                vec![]
            })
            .collect(),
        apns_topic: None,
        apns_type: None,
        suspended: tenant.suspended,
        suspended_reason: tenant.suspended_reason,
    };

    if providers.contains(&ProviderKind::Apns) {
        res.apns_topic = tenant.apns_topic;
        res.apns_type = tenant.apns_type;
    }

    debug!(
        tenant_id = %id,
        "requested tenant"
    );

    Ok(Json(res))
}
