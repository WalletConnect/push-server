pub mod delete_client;
pub mod health;
pub mod push_message;
pub mod register_client;

#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorLocation {
    Body,
    Query,
    Header
}

#[derive(serde::Serialize)]
pub struct ErrorReason {
    pub field: String,
    pub description: String,
    pub location: ErrorLocation
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub reasons: Vec<ErrorReason>,
}

pub fn new_error_response(reasons: Vec<ErrorReason>) -> ErrorResponse {
    ErrorResponse {
        status: "FAILED".to_string(),
        reasons,
    }
}

#[derive(serde::Serialize)]
pub struct SuccessResponse {
    status: String,
}

pub fn new_success_response() -> SuccessResponse {
    SuccessResponse {
        status: "OK".to_string(),
    }
}
