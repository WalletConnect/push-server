use crate::error::Error::{ClientAlreadyRegistered, EmptyField, ProviderNotAvailable};
use crate::error::Result;
use crate::handlers::Response;
use crate::state::AppState;
use crate::stores::client::Client;
use crate::stores::StoreError;
use axum::extract::{Json, Path, State};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RegisterBody {
    pub client_id: String,
    #[serde(rename = "type")]
    pub push_type: String,
    pub token: String,
}

pub async fn handler(
    Path(tenant): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterBody>,
) -> Result<Response> {
    // TODO (Harry): Rewrite provider check logic
    let push_type = body.push_type.as_str().try_into()?;
    let supported_providers = state.supported_providers();
    if !supported_providers.contains(&push_type) {
        return Err(ProviderNotAvailable(push_type.as_str().into()));
    }

    if body.token.is_empty() {
        return Err(EmptyField("token".to_string()));
    }

    let exists = match state.client_store.get_client(&tenant, &body.client_id).await {
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
        .create_client(
            &tenant,
            &body.client_id,
            Client {
                push_type,
                token: body.token,
            },
        )
        .await?;

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(1, &[]);
    }

    Ok(Response::default())
}
