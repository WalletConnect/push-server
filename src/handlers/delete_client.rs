use crate::handlers::{new_error_response, new_success_response, ErrorReason};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState<impl crate::store::ClientStore>>>,
) -> impl IntoResponse {
    let mut store = state.store.lock().unwrap();

    let exists = store.get_client(&id).await;
    if exists.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(new_error_response(vec![]))),
        );
    }

    if exists.unwrap().is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(new_error_response(vec![ErrorReason {
                field: "client_id".to_string(),
                description: "No client is registered with the supplied id".to_string(),
            }]))),
        );
    }

    let delete_result = store.delete_client(&id).await;
    if delete_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(new_error_response(vec![]))),
        );
    }

    if let Some(metrics) = &state.metrics {
        metrics.registered_webhooks.add(-1, &[]);
    }

    (StatusCode::OK, Json(json!(new_success_response())))
}
