use {
    crate::context::SingleTenantServerContext,
    echo_server::handlers::register_client::RegisterBody,
    random_string::generate,
    relay_rpc::domain::ClientId,
    std::sync::Arc,
    test_context::test_context,
};

#[test_context(SingleTenantServerContext)]
#[tokio::test]
async fn test_registration(ctx: &mut SingleTenantServerContext) {
    let charset = "1234567890";
    let random_client_id = ClientId::new(Arc::from(generate(12, charset)));
    let payload = RegisterBody {
        client_id: random_client_id.clone(),
        push_type: "noop".to_string(),
        token: "test".to_string(),
    };

    // Register client
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/clients", ctx.server.public_addr))
        .json(&payload)
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    // Update token
    let payload = RegisterBody {
        client_id: random_client_id,
        push_type: "noop".to_string(),
        token: "new_token".to_string(),
    };
    let response = client
        .post(format!("http://{}/clients", ctx.server.public_addr))
        .json(&payload)
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );
}

#[test_context(SingleTenantServerContext)]
#[tokio::test]
async fn test_deregistration(ctx: &mut SingleTenantServerContext) {
    let charset = "1234567890";
    let random_client_id = ClientId::new(Arc::from(generate(12, charset)));
    let payload = RegisterBody {
        client_id: random_client_id.clone(),
        push_type: "noop".to_string(),
        token: "test".to_string(),
    };

    let client = reqwest::Client::new();
    let register_response = client
        .post(format!("http://{}/clients", ctx.server.public_addr))
        .json(&payload)
        .send()
        .await
        .expect("Call failed");

    assert!(
        register_response.status().is_success(),
        "Failed to register client"
    );

    let client = reqwest::Client::new();
    let delete_response = client
        .delete(format!(
            "http://{}/clients/{}",
            ctx.server.public_addr, random_client_id
        ))
        .send()
        .await
        .expect("Call failed");

    let status = delete_response.status().clone();

    dbg!(&delete_response.text().await.unwrap());

    assert!(status.is_success(), "Failed to unregister client");
}
