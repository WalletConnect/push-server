use crate::error::Error;
use crate::handlers::{new_error_response, new_success_response, ErrorReason};
use crate::providers::{get_provider, PushProvider};
use crate::AppState;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct MessagePayload {
    message: String,
}

#[derive(Deserialize)]
pub struct PushMessageBody {
    id: String,
    payload: MessagePayload,
}

pub async fn handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState<impl crate::store::ClientStore>>>,
    Json(body): Json<PushMessageBody>,
) -> impl IntoResponse {
    let store = state.store.lock().unwrap();

    // TODO de-dup, and return accepted to already acknowledged notifications
    if body.id.as_str() == "0000-0000-0000-0000" {
        return (StatusCode::ACCEPTED, Json(json!(new_success_response())));
    }

    let client_result = store.get_client(&id);
    if let Err(_) = client_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(new_error_response(vec![]))),
        );
    }
    let client = client_result.unwrap();
    if client.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!(new_error_response(vec![ErrorReason {
                field: "id".to_string(),
                description: "No client found with the provided id".to_string(),
            }]))),
        );
    }

    let provider_name = &client.unwrap().push_type;
    let provider_result = get_provider(provider_name.clone(), &state);
    if let Err(err) = &provider_result {
        match err {
            Error::ProviderNotFound(_) => {
                // NOT POSSIBLE IN THEORY!
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!(new_error_response(vec![ErrorReason {
                        field: "client.provider".to_string(),
                        description: "The client's registered provider cannot be found."
                            .to_string(),
                    }]))),
                );
            }
            Error::ProviderNotAvailable(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!(new_error_response(vec![ErrorReason {
                        field: "client.provider".to_string(),
                        description: "The client's registered provider is not available."
                            .to_string(),
                    }]))),
                );
            }
            // Cannot be any other error
            _ => {}
        }
    }
    let mut provider = provider_result.unwrap();

    let res = provider
        .send_notification(client.unwrap().token.clone(), body.payload.message)
        .await;
    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(new_error_response(vec![]))),
        );
    }

    return (StatusCode::ACCEPTED, Json(json!(new_success_response())));
}
