use {
    crate::context::EchoServerContext,
    echo_server::handlers::create_tenant::TenantRegisterBody,
    random_string::generate,
    test_context::test_context,
};

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant(ctx: &mut EchoServerContext) {
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

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    // Get tenant
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr,
            random_tenant_id.clone()
        ))
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

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
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    // Get tenant again
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr,
            random_tenant_id.clone()
        ))
        .send()
        .await
        .expect("Call failed");

    // TODO: this should be changed to 404
    assert_eq!(
        response.status().as_u16(),
        400,
        "Response was not successful"
    );
}
