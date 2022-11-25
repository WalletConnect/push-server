use {
    crate::{
        error::Error::{
            FromRequestError,
            MissingAllSignatureHeader,
            MissingSignatureHeader,
            MissingTimestampHeader,
            ToBytesError,
        },
        state::State,
    },
    async_trait::async_trait,
    axum::{body, extract::FromRequest, http::Request},
    ed25519_dalek::{PublicKey, Signature, Verifier},
    tracing::span,
};

pub const SIGNATURE_HEADER_NAME: &str = "X-Ed25519-Signature";
pub const TIMESTAMP_HEADER_NAME: &str = "X-Ed25519-Timestamp";

pub struct RequireValidSignature<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for RequireValidSignature<T>
where
    // these bounds are required by
    // `async_trait`
    B: Send + 'static + body::HttpBody + From<hyper::body::Bytes>,
    B::Data: Send,
    S: Send + Sync + State,
    T: FromRequest<S, B>,
{
    type Rejection = crate::error::Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let s = span!(tracing::Level::DEBUG, "validate_signature");
        let _ = s.enter();

        let public_key = state.relay_client().public_key().await?;

        let (parts, body_raw) = req.into_parts();
        let bytes = hyper::body::to_bytes(body_raw)
            .await
            .map_err(|_| ToBytesError)?;
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
                Ok(T::from_request(req, state)
                    .await
                    .map(Self)
                    .map_err(|_| FromRequestError)?)
            }
            (Some(_), None) => Err(MissingTimestampHeader),
            (None, Some(_)) => Err(MissingSignatureHeader),
            (None, None) => Err(MissingAllSignatureHeader),
            _ => Err(MissingAllSignatureHeader),
        }
    }
}

pub async fn signature_is_valid(
    signature: &str,
    timestamp: &str,
    body: &str,
    public_key: &PublicKey,
) -> Result<bool, crate::error::Error> {
    let sig_body = format!("{}.{}.{}", timestamp, body.len(), body);

    let sig_bytes = hex::decode(signature)?;
    let sig = Signature::from_bytes(&sig_bytes)?;

    Ok(public_key.verify(sig_body.as_bytes(), &sig).is_ok())
}
