use {
    crate::{context::EchoServerContext, functional::multitenant::generate_random_tenant_id},
    echo_server::handlers::create_tenant::TenantRegisterBody,
    test_context::test_context,
};

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_register_get_delete(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register tenant
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Get tenant
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
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
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Get tenant again
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .send()
        .await
        .expect("Call failed");
    // TODO: this should be changed to 404
    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
}
