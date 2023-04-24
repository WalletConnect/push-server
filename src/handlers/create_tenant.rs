use {
    crate::{
        error::{Error, Error::InvalidProjectId},
        handlers::validate_tenant_request,
        increment_counter,
        request_id::get_req_id,
        state::AppState,
        stores::tenant::TenantUpdateParams,
    },
    axum::{extract::State, http::HeaderMap, Json},
    cerberus::registry::RegistryClient,
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

    if let Some(project) = project {
        validate_tenant_request(
            &state.registry_client,
            &state.gotrue_client,
            &headers,
            body.id.clone(),
            Some(project),
        )
        .await?;
    } else {
        return Err(InvalidProjectId(body.id));
    }

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
