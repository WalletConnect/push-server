pub mod health;
pub mod register_client;
pub mod delete_client;
pub mod push_message;

#[derive(serde::Serialize)]
pub struct ErrorReason {
    pub field: String,
    pub description: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub reasons: Vec<ErrorReason>
}

pub fn new_error_response(reasons: Vec<ErrorReason>) -> ErrorResponse {
    ErrorResponse {
        status: "FAILED".to_string(),
        reasons
    }
}

#[derive(serde::Serialize)]
pub struct SuccessResponse {
    status: String
}

pub fn new_success_response() -> SuccessResponse {
    SuccessResponse {
        status: "OK".to_string()
    }
}