use {
    crate::context::EchoServerContext,
    echo_server::handlers::{
        push_message::{MessagePayload, PushMessageBody},
        register_client::RegisterBody,
    },
    relay_rpc::{
        auth::{
            ed25519_dalek::Keypair,
            rand::{rngs::StdRng, SeedableRng},
        },
        domain::{ClientId, DecodedClientId},
    },
    test_context::test_context,
    uuid::Uuid,
};

#[test_context(EchoServerContext)]
#[tokio::test]
async fn test_push(ctx: &mut EchoServerContext) {
    let mut rng = StdRng::from_entropy();
    let keypair = Keypair::generate(&mut rng);

    let random_client_id = DecodedClientId(*keypair.public_key().as_bytes());
    let client_id = ClientId::from(random_client_id);

    let jwt = relay_rpc::auth::AuthToken::new(client_id.value().clone())
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
    };

    // Register client
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/clients", ctx.server.public_addr))
        .json(&payload)
        .header("Authorization", jwt.clone())
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    // Push
    let push_message_id = Uuid::new_v4().to_string();
    let topic = Uuid::new_v4().to_string();
    let blob = Uuid::new_v4().to_string();
    let push_message_payload = MessagePayload {
        topic: Some(topic.into()),
        blob: blob.to_string(),
        flags: 0,
    };
    let payload = PushMessageBody {
        id: push_message_id.clone(),
        payload: push_message_payload,
    };

    // Push
    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "http://{}/clients/{}",
            ctx.server.public_addr,
            client_id.clone()
        ))
        .json(&payload)
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    // Push the same payload again and ensure it's deduped
    let client = reqwest::Client::new();
    let already_pushed_status_code = 200;
    let response = client
        .post(format!(
            "http://{}/clients/{}",
            ctx.server.public_addr,
            client_id.clone()
        ))
        .json(&payload)
        .send()
        .await
        .expect("Call failed");

    assert_eq!(
        response.status().as_u16(),
        already_pushed_status_code,
        "Response was not successful"
    );
}
