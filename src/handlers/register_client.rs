use {
    crate::{
        authentication::Jwt,
        error::{
            Error::{EmptyField, InvalidAuthentication, ProviderNotAvailable},
            Result,
        },
        handlers::{authenticate_client, Response, DECENTRALIZED_IDENTIFIER_PREFIX},
        increment_counter,
        log::prelude::*,
        state::AppState,
        stores::client::Client,
    },
    axum::{
        extract::{Json, Path, State as StateExtractor},
        http::HeaderMap,
    },
    hyper::header::AUTHORIZATION,
    relay_rpc::domain::ClientId,
    serde::{Deserialize, Serialize},
    std::{collections::HashSet, sync::Arc},
};

#[derive(Serialize, Deserialize)]
pub struct RegisterBody {
    pub client_id: ClientId,
    #[serde(rename = "type")]
    pub push_type: String,
    pub token: String,
}

pub async fn handler(
    Path(tenant_id): Path<String>,
    StateExtractor(state): StateExtractor<Arc<AppState>>,
    Json(body): Json<RegisterBody>,
    headers: HeaderMap,
) -> Result<Response> {
    if !authenticate_client(headers, |client_id| {
        if let Some(client_id) = client_id {
            &client_id == &body.client_id
        } else {
            false
        }
    })? {
        return Err(InvalidAuthentication);
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

    let client_id = body
        .client_id
        .trim_start_matches(DECENTRALIZED_IDENTIFIER_PREFIX);

    state
        .client_store
        .create_client(&tenant_id, client_id, Client {
            push_type,
            token: body.token,
        })
        .await?;

    info!(
        "client registered for tenant ({}) using {}",
        tenant_id, body.push_type
    );

    increment_counter!(state.metrics, registered_clients);

    Ok(Response::default())
}
