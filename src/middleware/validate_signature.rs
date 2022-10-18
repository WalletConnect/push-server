use crate::handlers::{new_error_response, ErrorLocation, ErrorReason};
use crate::state::State;
use async_trait::async_trait;
use axum::body;
use axum::extract::{FromRequest, Json};
use axum::http::{Request, StatusCode};
use ed25519_dalek::{PublicKey, Signature, Verifier};
use serde_json::json;

const SIGNATURE_HEADER_NAME: &str = "X-Ed25519-Signature";
const TIMESTAMP_HEADER_NAME: &str = "X-Ed25519-Timestamp";

pub struct RequireValidSignature<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for RequireValidSignature<T>
where
    // these bounds are required by `async_trait`
    B: Send + 'static + body::HttpBody + From<hyper::body::Bytes>,
    B::Data: Send,
    S: Send + Sync + State<sqlx::PgPool, sqlx::PgPool>,
    T: FromRequest<S, B>,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let public_key = match state.relay_client().public_key().await {
            Ok(key) => key,
            Err(_) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(new_error_response(vec![]))),
                ));
            }
        };

        let (parts, body_raw) = req.into_parts();
        let bytes = hyper::body::to_bytes(body_raw).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(new_error_response(vec![]))),
            )
        })?;
        let body = String::from_utf8_lossy(&bytes);

        let signature_header = parts
            .headers
            .get(SIGNATURE_HEADER_NAME)
            .and_then(|header| header.to_str().ok());

        let timestamp_header = parts
            .headers
            .get(TIMESTAMP_HEADER_NAME)
            .and_then(|header| header.to_str().ok());

        match (signature_header, timestamp_header) {
            (Some(signature), Some(timestamp))
                if signature_is_valid(signature, timestamp, &body, &public_key).await? =>
            {
                let req = Request::<B>::from_parts(parts, bytes.into());
                T::from_request(req, state).await.map(Self).map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!(new_error_response(vec![ErrorReason {
                            field: TIMESTAMP_HEADER_NAME.to_string(),
                            description: "Missing timestamp".to_string(),
                            location: ErrorLocation::Header
                        }]))),
                    )
                })
            }
            (Some(_), None) => Err((
                StatusCode::UNAUTHORIZED,
                Json(json!(new_error_response(vec![ErrorReason {
                    field: TIMESTAMP_HEADER_NAME.to_string(),
                    description: "Missing timestamp".to_string(),
                    location: ErrorLocation::Header
                }]))),
            )),
            (None, Some(_)) => Err((
                StatusCode::UNAUTHORIZED,
                Json(json!(new_error_response(vec![ErrorReason {
                    field: SIGNATURE_HEADER_NAME.to_string(),
                    description: "Missing signature".to_string(),
                    location: ErrorLocation::Header
                },]))),
            )),
            (None, None) => Err((
                StatusCode::UNAUTHORIZED,
                Json(json!(new_error_response(vec![
                    ErrorReason {
                        field: SIGNATURE_HEADER_NAME.to_string(),
                        description: "Missing signature".to_string(),
                        location: ErrorLocation::Header
                    },
                    ErrorReason {
                        field: TIMESTAMP_HEADER_NAME.to_string(),
                        description: "Missing timestamp".to_string(),
                        location: ErrorLocation::Header
                    }
                ]))),
            )),
            _ => Err((
                StatusCode::UNAUTHORIZED,
                Json(json!(new_error_response(vec![]))),
            )),
        }
    }
}

pub async fn signature_is_valid(
    signature: &str,
    timestamp: &str,
    body: &str,
    public_key: &PublicKey,
) -> Result<bool, (StatusCode, Json<serde_json::Value>)> {
    let sig_body = format!("{}.{}.{}", timestamp, body.len(), body);

    let sig_bytes_result = hex::decode(signature);
    if sig_bytes_result.is_err() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!(new_error_response(vec![ErrorReason {
                field: SIGNATURE_HEADER_NAME.to_string(),
                description: "Signature is not valid hex".to_string(),
                location: ErrorLocation::Header
            }]))),
        ));
    }
    let sig_bytes = sig_bytes_result.unwrap();
    let sig = Signature::from_bytes(&sig_bytes);
    if sig.is_err() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!(new_error_response(vec![ErrorReason {
                field: SIGNATURE_HEADER_NAME.to_string(),
                description: "Failed to parse signature from bytes".to_string(),
                location: ErrorLocation::Header
            }]))),
        ));
    }

    Ok(public_key
        .verify(sig_body.as_bytes(), &sig.unwrap())
        .is_ok())
}
