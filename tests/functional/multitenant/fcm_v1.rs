use {
    crate::{context::EchoServerContext, functional::multitenant::ClaimsForValidation},
    echo_server::handlers::create_tenant::TenantRegisterBody,
    jsonwebtoken::{encode, EncodingKey, Header},
    random_string::generate,
    std::{env, time::SystemTime},
    test_context::test_context,
};

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_fcm_v1_valid(ctx: &mut EchoServerContext) {
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
        aud: "authenticated".to_string(),
        role: "authenticated".to_string(),
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
    let register_response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .json(&payload)
        .header("AUTHORIZATION", jwt_token.clone())
        .send()
        .await
        .expect("Call failed");
    assert_eq!(register_response.status(), reqwest::StatusCode::OK);

    // Send valid API Key
    let credentials = env::var("ECHO_TEST_FCM_V1_CREDENTIALS").unwrap();
    let form = reqwest::multipart::Form::new().text("credentials", credentials);

    let response_fcm_update = client
        .post(format!(
            "http://{}/tenants/{}/fcm_v1",
            ctx.server.public_addr, &random_tenant_id
        ))
        .header("AUTHORIZATION", jwt_token.clone())
        .multipart(form)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response_fcm_update.status(), reqwest::StatusCode::OK);
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_fcm_v1_bad(ctx: &mut EchoServerContext) {
    let charset = "1234567890";
    let random_tenant_id = generate(12, charset);
    let payload = TenantRegisterBody {
        id: random_tenant_id.clone(),
    };

    // Register tenant
    let client = reqwest::Client::new();
    client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .json(&payload)
        .send()
        .await
        .expect("Call failed");

    // Send invalid API Key
    let form = reqwest::multipart::Form::new().text("credentials", "invalid-key");

    let response = client
        .post(format!(
            "http://{}/tenants/{}/fcm_v1",
            ctx.server.public_addr, &random_tenant_id
        ))
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(!response.status().is_success(), "Response was successful");
}
