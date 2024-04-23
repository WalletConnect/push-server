/// Tests against the handlers
use {crate::context::EchoServerContext, serde::Serialize, test_context::test_context};

mod apns;
mod fcm;
mod fcm_v1;
mod tenancy;

/// Struct to hold claims for JWT validation
#[derive(Serialize)]
pub struct ClaimsForValidation {
    sub: String,
    exp: usize,
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn test_health(ctx: &mut EchoServerContext) {
    let body = reqwest::get(format!("http://{}/health", ctx.server.public_addr))
        .await
        .expect("Failed to call /health")
        .status();
    assert!(body.is_success());
}

pub fn generate_random_tenant_id() -> (String, String) {
    let charset = "1234567890";
    let random_tenant_id = generate(12, charset);
    let unix_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let token_claims = ClaimsForValidation {
        sub: random_tenant_id.clone(),
        exp: unix_timestamp + 60 * 60, // Add an hour for expiration
    };
    let jwt_token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(ctx.config.jwt_secret.as_bytes()),
    )
    .expect("Failed to encode jwt token");
    (random_tenant_id, jwt_token)
}
