use {
    crate::context::EchoServerContext,
    jsonwebtoken::{encode, EncodingKey, Header},
    random_string::generate,
    serde::Serialize,
    std::time::SystemTime,
    test_context::test_context,
};

#[cfg(feature = "apns_tests")]
mod apns;
#[cfg(feature = "fcm_tests")]
mod fcm;
#[cfg(feature = "fcmv1_tests")]
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

pub fn generate_random_tenant_id(jwt_secret: &str) -> (String, String) {
    let charset = "1234567890";
    let tenant_id = generate(12, charset);
    let unix_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let token_claims = ClaimsForValidation {
        sub: tenant_id.clone(),
        exp: unix_timestamp + 60 * 60, // Add an hour for expiration
    };
    let jwt_token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .expect("Failed to encode jwt token");
    (tenant_id, jwt_token)
}
