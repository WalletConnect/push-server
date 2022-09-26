use crate::store::Client;
use crate::{
    handlers::{new_error_response, new_success_response, ErrorReason},
    state::AppState,
};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use itertools::Itertools;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RegisterBody {
    pub client_id: String,
    #[serde(rename = "type")]
    pub push_type: String,
    pub token: String,
}

pub async fn handler(
    State(state): State<Arc<AppState<impl crate::store::ClientStore>>>,
    Json(body): Json<RegisterBody>,
) -> impl IntoResponse {
    let push_type = body.push_type.as_str().try_into();
    let supported_providers = state.supported_providers();
    let push_type = match push_type {
        Ok(provider) if supported_providers.contains(&provider) => provider,

        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!(new_error_response(vec![ErrorReason {
                    field: "type".to_string(),
                    description: format!(
                        "Invalid Push Service, must be one of: {}",
                        supported_providers
                            .iter()
                            .map(|provider| provider.as_str())
                            .join(", ")
                    ),
                }]))),
            )
        }
    };

    if body.token.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(new_error_response(vec![ErrorReason {
                field: "token".to_string(),
                description: "The `token` field must not be empty".to_string(),
            }]))),
        );
    }

    let internal_server_error = new_error_response(vec![]);

    let exists = state.store.get_client(&body.client_id).await;
    if exists.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(&internal_server_error)),
        );
    }

    if exists.unwrap().is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(new_error_response(vec![ErrorReason {
                field: "client_id".to_string(),
                description: "A client is already registered with this id".to_string(),
            }]))),
        );
    }

    let create_client_res = state
        .store
        .create_client(
            &body.client_id,
            Client {
                push_type,
                token: body.token,
            },
        )
        .await;

    if create_client_res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(&internal_server_error)),
        );
    }

    // TODO Register webhook with relay.

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(1, &[]);
    }

    (StatusCode::OK, Json(json!(new_success_response())))
}
