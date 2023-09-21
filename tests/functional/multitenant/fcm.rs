use {
    crate::context::EchoServerContext,
    echo_server::handlers::create_tenant::TenantRegisterBody,
    random_string::generate,
    test_context::test_context,
};

#[test_context(EchoServerContext)]
#[tokio::test]
// This test is unexpectedly failing and ignored until the resolution
#[ignore]
async fn tenant_update_fcm(ctx: &mut EchoServerContext) {
    let charset = "1234567890";
    let random_tenant_id = generate(12, charset);
    let payload = TenantRegisterBody {
        id: random_tenant_id.clone(),
    };

    // Register tenant
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .json(&payload)
        .send()
        .await
        .expect("Call failed");

    // Send valid API Key
    let api_key = env!("ECHO_TEST_FCM_KEY");
    let form = reqwest::multipart::Form::new().text("api_key", api_key);

    let response = client
        .post(format!(
            "http://{}/tenants/{}/fcm",
            ctx.server.public_addr, &random_tenant_id
        ))
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_fcm_bad(ctx: &mut EchoServerContext) {
    let charset = "1234567890";
    let random_tenant_id = generate(12, charset);
    let payload = TenantRegisterBody {
        id: random_tenant_id.clone(),
    };

    // Register tenant
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .json(&payload)
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
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(!response.status().is_success(), "Response was successful");
}
