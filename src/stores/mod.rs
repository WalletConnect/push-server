use crate::handlers::{new_error_response, ErrorLocation};
use axum::response::{IntoResponse, Response};
use axum::Json;
use echo_server::handlers::ErrorReason;
use hyper::StatusCode;
use serde_json::json;

pub mod client;
pub mod notification;

type Result<T> = std::result::Result<T, StoreError>;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    /// Not found error, params are entity name and identifier
    #[error("Cannot find {0} with specified identifier {1}")]
    NotFound(String, String),
}

impl IntoResponse for StoreError {
    fn into_response(self) -> Response {
        match self {
            StoreError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(new_error_response(vec![]))),
            ),
            StoreError::NotFound(entity, id) => (
                StatusCode::NOT_FOUND,
                Json(json!(new_error_response(vec![ErrorReason {
                    field: format!("{}.id", &entity),
                    description: format!("Cannot find {} with specified identifier {}", entity, id),
                    location: ErrorLocation::Body // TODO evaluate if correct location
                }]))),
            ),
        }
        .into()
    }
}
