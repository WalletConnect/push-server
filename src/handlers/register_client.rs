use {
    crate::{
        error::{
            Error::{
                ClientAlreadyRegistered,
                EmptyField,
                IncludedTenantIdWhenNotNeeded,
                ProviderNotAvailable,
            },
            Result,
        },
        handlers::Response,
        state::{AppState, State},
        stores::{client::Client, StoreError},
    },
    axum::extract::{Json, Path, State as StateExtractor},
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Serialize, Deserialize)]
pub struct RegisterBody {
    pub client_id: String,
    #[serde(rename = "type")]
    pub push_type: String,
    pub token: String,
}

pub async fn handler(
    Path(tenant_id): Path<String>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    Json(body): Json<RegisterBody>,
) -> Result<Response> {
    if state.config.default_tenant_id != tenant_id && !state.is_multitenant() {
        return Err(IncludedTenantIdWhenNotNeeded);
    }

    let push_type = body.push_type.as_str().try_into()?;
    let tenant = state.tenant_store.get_tenant(&tenant_id).await?;
    let supported_providers = tenant.providers();
    if !supported_providers.contains(&push_type) {
        return Err(ProviderNotAvailable(push_type.into()));
    }

    if body.token.is_empty() {
        return Err(EmptyField("token".to_string()));
    }

    let exists = match state
        .client_store
        .get_client(&tenant_id, &body.client_id)
        .await
    {
        Ok(_) => true,
        Err(e) => match e {
            StoreError::Database(db_error) => {
                return Err(db_error.into());
            }
            StoreError::NotFound(_, _) => false,
        },
    };

    if exists {
        return Err(ClientAlreadyRegistered);
    }

    state
        .client_store
        .create_client(&tenant_id, &body.client_id, Client {
            push_type,
            token: body.token,
        })
        .await?;

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(1, &[]);
    }

    Ok(Response::default())
}
