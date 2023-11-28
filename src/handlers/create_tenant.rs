#[cfg(feature = "cloud")]
use cerberus::registry::RegistryClient;
use {
    crate::{
        error::{Error, Error::InvalidProjectId},
        handlers::validate_tenant_request,
        increment_counter,
        log::prelude::*,
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
    #[cfg(feature = "cloud")]
    let (valid_id, project) = {
        let project_id = body.id.clone();

        let response = state.registry_client.project_data(&project_id).await?;

        if let Some(project) = response {
            // TODO potentially more validation in future
            // Project passed forwards for JWT verification later
            (project.is_enabled, Some(project))
        } else {
            (false, None)
        }
    };

    // When not using the cloud app all Ids are valid
    #[cfg(not(feature = "cloud"))]
    let valid_id = true;

    if !valid_id {
        return Err(InvalidProjectId(body.id));
    }

    #[cfg(feature = "cloud")]
    if let Some(project) = project {
        if let Err(e) = validate_tenant_request(
            &state.registry_client,
            &state.gotrue_client,
            &headers,
            body.id.clone(),
            Some(project),
        )
        .await
        {
            error!(
                tenant_id = %body.id,
                err = ?e,
                "JWT verification failed"
            );
            return Err(e);
        }
    } else {
        return Err(InvalidProjectId(body.id));
    }

    #[cfg(not(feature = "cloud"))]
    if let Err(e) = validate_tenant_request(&state.gotrue_client, &headers) {
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

    info!(
        tenant_id = %tenant.id,
        "new tenant"
    );

    Ok(Json(TenantRegisterResponse {
        url: format!("{}/{}", state.config.public_url, tenant.id),
    }))
}
