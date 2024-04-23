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
async fn tenant_update_fcm_valid(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id();

    // Register tenant
    let client = reqwest::Client::new();
    let register_response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Call failed");
    assert_eq!(register_response.status(), reqwest::StatusCode::OK);

    // Send valid API Key
    let api_key = env::var("ECHO_TEST_FCM_KEY").unwrap();
    let form = reqwest::multipart::Form::new().text("api_key", api_key);

    let response_fcm_update = client
        .post(format!(
            "http://{}/tenants/{}/fcm",
            ctx.server.public_addr, &random_tenant_id
        ))
        .bearer_auth(jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response_fcm_update.status(), reqwest::StatusCode::OK);
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_fcm_bad(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id();

    // Register tenant
    let client = reqwest::Client::new();
    client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Call failed");

    // Send invalid API Key
    let form = reqwest::multipart::Form::new().text("api_key", "invalid-key");

    let response = client
        .post(format!(
            "http://{}/tenants/{}/fcm",
            ctx.server.public_addr, &random_tenant_id
        ))
        .bearer_auth(jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(!response.status().is_success(), "Response was successful");
}
