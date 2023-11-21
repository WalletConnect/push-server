use {
    crate::context::EchoServerContext,
    echo_server::{
        handlers::{push_message::PushMessageBody, register_client::RegisterBody},
        providers::{LegacyPushMessage, MessagePayload, RawPushMessage},
    },
    hyper::StatusCode,
    relay_rpc::{
        auth::{
            ed25519_dalek::Keypair,
            rand::{rngs::StdRng, SeedableRng},
        },
        domain::{ClientId, DecodedClientId},
    },
    std::sync::Arc,
    test_context::test_context,
    uuid::Uuid,
    wiremock::{http::Method, matchers::method, Mock, MockServer, ResponseTemplate},
};

async fn create_client(ctx: &mut EchoServerContext, always_raw: bool) -> (ClientId, MockServer) {
    let mut rng = StdRng::from_entropy();
    let keypair = Keypair::generate(&mut rng);

    let random_client_id = DecodedClientId(*keypair.public_key().as_bytes());
    let client_id = ClientId::from(random_client_id);

    let jwt = relay_rpc::auth::AuthToken::new(client_id.value().to_string())
        .aud(format!(
            "http://127.0.0.1:{}",
            ctx.server.public_addr.port()
        ))
        .as_jwt(&keypair)
        .unwrap()
        .to_string();

    let mock_server = {
        let mock_server = MockServer::start().await;
        Mock::given(method(Method::Get))
            .respond_with(ResponseTemplate::new(StatusCode::OK))
            .expect(1)
            .mount(&mock_server)
            .await;
        mock_server
    };
    let token = mock_server.uri();

    let payload = RegisterBody {
        client_id: client_id.clone(),
        push_type: "noop".to_string(),
        token: token.clone(),
        always_raw: Some(always_raw),
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

    (client_id, mock_server)
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn test_push(ctx: &mut EchoServerContext) {
    let (client_id, _mock_server) = create_client(ctx, false).await;

    // Push
    let push_message_id = Uuid::new_v4().to_string().into();
    let topic = Uuid::new_v4().to_string().into();
    let blob = Uuid::new_v4().to_string().into();
    let push_message_payload = MessagePayload {
        topic,
        blob,
        flags: 0,
    };
    let payload = PushMessageBody {
        raw: None,
        legacy: Some(LegacyPushMessage {
            id: push_message_id,
            payload: push_message_payload,
        }),
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

#[test_context(EchoServerContext)]
#[tokio::test]
async fn test_push_multiple_clients(ctx: &mut EchoServerContext) {
    let (client_id1, _mock_server1) = create_client(ctx, false).await;
    let (client_id2, _mock_server2) = create_client(ctx, false).await;

    // Push
    let push_message_id: Arc<str> = Uuid::new_v4().to_string().into();
    let topic = Uuid::new_v4().to_string().into();
    let blob = Uuid::new_v4().to_string().into();
    let push_message_payload = MessagePayload {
        topic,
        blob,
        flags: 0,
    };
    let payload = PushMessageBody {
        raw: None,
        legacy: Some(LegacyPushMessage {
            id: push_message_id.clone(),
            payload: push_message_payload,
        }),
    };

    // Push client 1
    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "http://{}/clients/{}",
            ctx.server.public_addr,
            client_id1.clone()
        ))
        .json(&payload)
        .send()
        .await
        .expect("Call failed");
    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    // Push client 2
    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "http://{}/clients/{}",
            ctx.server.public_addr,
            client_id2.clone()
        ))
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
async fn test_push_always_raw(ctx: &mut EchoServerContext) {
    // Create client with always_raw = true
    let (client_id, _mock_server) = create_client(ctx, true).await;

    let push_message_id = Uuid::new_v4().to_string().into();
    let topic: Arc<str> = Uuid::new_v4().to_string().into();
    let blob: Arc<str> = Uuid::new_v4().to_string().into();
    let push_message_payload = MessagePayload {
        topic: topic.clone(),
        blob: blob.clone(),
        flags: 0,
    };
    let client = reqwest::Client::new();

    // Send push with WRONG payload without necessary fields for always_raw
    let wrong_payload = PushMessageBody {
        raw: None,
        legacy: Some(LegacyPushMessage {
            id: push_message_id,
            payload: push_message_payload,
        }),
    };
    let response = client
        .post(format!(
            "http://{}/clients/{}",
            ctx.server.public_addr,
            client_id.clone()
        ))
        .json(&wrong_payload)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Send push with good payload without necessary fields for always_raw
    let good_payload = PushMessageBody {
        raw: Some(RawPushMessage {
            topic,
            tag: 1100,
            message: blob,
        }),
        legacy: None,
    };
    let response = client
        .post(format!(
            "http://{}/clients/{}",
            ctx.server.public_addr,
            client_id.clone()
        ))
        .json(&good_payload)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response.status(), StatusCode::ACCEPTED);
}
