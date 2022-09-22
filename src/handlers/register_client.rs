use crate::handlers::{new_error_response, new_success_response, ErrorReason};
use crate::store::Client;
use crate::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
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
    let internal_server_error = new_error_response(vec![]);

    if !vec!["fcm", "apns"].contains(&&*body.push_type.to_lowercase()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(new_error_response(vec![ErrorReason {
                field: "type".to_string(),
                description: "Invalid Push Service, must be one of: fcm, apns".to_string(),
            }]))),
        );
    }

    if body.token.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(new_error_response(vec![ErrorReason {
                field: "token".to_string(),
                description: "The `token` field must not be empty".to_string(),
            }]))),
        );
    }

    let mut store = state.store.lock().unwrap();
    let exists = store.get_client(&body.client_id);
    if let Err(_) = exists {
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

    if let Err(_) = store.create_client(
        body.client_id.clone(),
        Client {
            push_type: body.push_type,
            token: body.token,
        },
    ) {
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