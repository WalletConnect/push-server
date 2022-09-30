use crate::handlers::{new_error_response, ErrorReason};
use crate::state::AppState;
use crate::store::ClientStore;
use axum::body::{BoxBody, Full};
use axum::{body, http::Request, middleware::Next, response::Response};
use ed25519_dalek::{PublicKey, Signature, Verifier};
use serde_json::json;
use sqlx::types::JsonValue;

const SIGNATURE_HEADER_NAME: &str = "X-Ed25519-Signature";
const TIMESTAMP_HEADER_NAME: &str = "X-Ed25519-Timestamp";

pub async fn validate_signature(
    req: Request<BoxBody>,
    next: Next<BoxBody>,
    mut state: AppState<impl ClientStore>,
) -> Result<Response, (axum::http::StatusCode, JsonValue)> {
    let public_key_result = state.relay_client.public_key().await;
    if public_key_result.is_err() {
        return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            json!(new_error_response(vec![])),
        ));
    }

    let (parts, body_raw) = req.into_parts();
    let body_bytes = hyper::body::to_bytes(body_raw).await.map_err(|_| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            json!(new_error_response(vec![])),
        )
    })?;
    let body = String::from_utf8(body_bytes.clone().to_vec()).map_err(|_| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            json!(new_error_response(vec![])),
        )
    })?;

    let signature_header = &parts
        .headers
        .get(SIGNATURE_HEADER_NAME)
        .and_then(|header| header.to_str().ok());

    let timestamp_header = &parts
        .headers
        .get(TIMESTAMP_HEADER_NAME)
        .and_then(|header| header.to_str().ok());

    match (signature_header, timestamp_header) {
        (Some(signature), Some(timestamp))
            if signature_is_valid(signature, timestamp, body, &public_key_result.unwrap())
                .await? =>
        {
            Ok(next
                .run(Request::from_parts(
                    parts,
                    body::boxed(Full::from(body_bytes)),
                ))
                .await)
        }
        _ => Err((
            axum::http::StatusCode::UNAUTHORIZED,
            json!(new_error_response(vec![ErrorReason {
                field: SIGNATURE_HEADER_NAME.to_string(),
                description: "Invalid signature".to_string(),
            }])),
        )),
    }
}

async fn signature_is_valid(
    signature: &str,
    timestamp: &str,
    body: String,
    public_key: &PublicKey,
) -> Result<bool, (axum::http::StatusCode, JsonValue)> {
    let sig_body = format!("{}.{}.{}", timestamp, body.len(), body);

    let sig_bytes_result = hex::decode(signature);
    if sig_bytes_result.is_err() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            json!(new_error_response(vec![ErrorReason {
                field: SIGNATURE_HEADER_NAME.to_string(),
                description: "Signature is not valid hex".to_string(),
            }])),
        ));
    }
    let sig_bytes = sig_bytes_result.unwrap();
    let sig = Signature::from_bytes(&sig_bytes);
    if sig.is_err() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            json!(new_error_response(vec![ErrorReason {
                field: SIGNATURE_HEADER_NAME.to_string(),
                description: "Failed to parse signature from bytes".to_string(),
            }])),
        ));
    }

    Ok(matches!(
        public_key.verify(sig_body.as_bytes(), &sig.unwrap()),
        Ok(_)
    ))
}
