use opentelemetry::Context;
use {
    crate::{
        error::{
            Error::{EmptyField, ProviderNotAvailable},
            Result,
        },
        handlers::Response,
        state::AppState,
        stores::client::Client,
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
    let push_type = body.push_type.as_str().try_into()?;
    let tenant = state.tenant_store.get_tenant(&tenant_id).await?;
    let supported_providers = tenant.providers();
    if !supported_providers.contains(&push_type) {
        return Err(ProviderNotAvailable(push_type.into()));
    }

    if body.token.is_empty() {
        return Err(EmptyField("token".to_string()));
    }

    state
        .client_store
        .create_client(&tenant_id, &body.client_id, Client {
            push_type,
            token: body.token,
        })
        .await?;

    if let Some(metrics) = &state.metrics {
        metrics.registered_clients.add(&Context::current(), 1, &[]);
    }

    Ok(Response::default())
}
