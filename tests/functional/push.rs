use {
    crate::context::SingleTenantServerContext,
    echo_server::handlers::{
        push_message::{MessagePayload, PushMessageBody},
        register_client::RegisterBody,
    },
    random_string::generate,
    relay_rpc::domain::ClientId,
    std::sync::Arc,
    test_context::test_context,
    uuid::Uuid,
};

#[test_context(SingleTenantServerContext)]
#[tokio::test]
async fn test_push(ctx: &mut SingleTenantServerContext) {
    let charset = "1234567890";
    let random_client_id = ClientId(Arc::from(generate(12, charset)));
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
            random_client_id.clone()
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
            random_client_id.clone()
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
