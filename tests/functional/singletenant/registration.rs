use {
    crate::context::EchoServerContext,
    echo_server::handlers::register_client::RegisterBody,
    ed25519_dalek::SigningKey,
    relay_rpc::domain::{ClientId, DecodedClientId},
    test_context::test_context,
};

#[test_context(EchoServerContext)]
#[tokio::test]
async fn test_registration(ctx: &mut EchoServerContext) {
    let keypair = SigningKey::generate(&mut rand::thread_rng());

    let random_client_id = DecodedClientId::from_key(&keypair.verifying_key());
    let client_id = ClientId::from(random_client_id);
    let payload = RegisterBody {
        client_id: client_id.clone(),
        push_type: "noop".to_string(),
        token: "test".to_string(),
        always_raw: Some(false),
    };

    let jwt = relay_rpc::auth::AuthToken::new(client_id.value().to_string())
        .aud(format!(
            "http://127.0.0.1:{}",
            ctx.server.public_addr.port()
        ))
        .as_jwt(&keypair)
        .unwrap()
        .to_string();

    // Register client
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/clients", ctx.server.public_addr))
        .header("Authorization", jwt.clone())
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
        client_id,
        push_type: "noop".to_string(),
        token: "new_token".to_string(),
        always_raw: Some(false),
    };
    let response = client
        .post(format!("http://{}/clients", ctx.server.public_addr))
        .header("Authorization", jwt)
        .json(&payload)
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
async fn test_deregistration(ctx: &mut EchoServerContext) {
    let keypair = SigningKey::generate(&mut rand::thread_rng());

    let random_client_id = DecodedClientId::from_key(&keypair.verifying_key());
    let client_id = ClientId::from(random_client_id);

    let jwt = relay_rpc::auth::AuthToken::new(client_id.value().to_string())
        .aud(format!(
            "http://127.0.0.1:{}",
            ctx.server.public_addr.port()
        ))
        .as_jwt(&keypair)
        .unwrap()
        .to_string();

    let payload = RegisterBody {
        client_id: client_id.clone(),
        push_type: "noop".to_string(),
        token: "test".to_string(),
        always_raw: Some(false),
    };

    let client = reqwest::Client::new();
    let register_response = client
        .post(format!("http://{}/clients", ctx.server.public_addr))
        .json(&payload)
        .header("Authorization", jwt.clone())
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
            ctx.server.public_addr, client_id
        ))
        .header("Authorization", jwt)
        .send()
        .await
        .expect("Call failed")
        .status();

    assert!(delete_response.is_success(), "Failed to unregister client");
}
