use {
    crate::{context::EchoServerContext, functional::multitenant::ClaimsForValidation},
    echo_server::handlers::create_tenant::TenantRegisterBody,
    jsonwebtoken::{encode, EncodingKey, Header},
    random_string::generate,
    std::time::SystemTime,
    test_context::test_context,
};

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_register_get_delete(ctx: &mut EchoServerContext) {
    let charset = "1234567890";
    let random_tenant_id = generate(12, charset);
    let payload = TenantRegisterBody {
        id: random_tenant_id.clone(),
    };
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

    // Register tenant
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .json(&payload)
        .header("AUTHORIZATION", jwt_token.clone())
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Get tenant
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr,
            random_tenant_id.clone()
        ))
        .header("AUTHORIZATION", jwt_token.clone())
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Check for CORS
    assert!(response
        .headers()
        .contains_key("Access-Control-Allow-Origin"));
    let allowed_origins = response
        .headers()
        .get("Access-Control-Allow-Origin")
        .unwrap();
    assert_eq!(allowed_origins.to_str().unwrap(), "*");

    // Delete tenant
    let response = client
        .delete(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr,
            random_tenant_id.clone()
        ))
        .header("AUTHORIZATION", jwt_token.clone())
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Get tenant again
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr,
            random_tenant_id.clone()
        ))
        .header("AUTHORIZATION", jwt_token.clone())
        .send()
        .await
        .expect("Call failed");
    // TODO: this should be changed to 404
    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
}
