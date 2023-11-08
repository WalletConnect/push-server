use {
    crate::{
        context::StoreContext,
        functional::stores::{gen_id, TENANT_ID},
    },
    echo_server::{providers::ProviderKind, stores::client::Client},
    test_context::test_context,
};

#[test_context(StoreContext)]
#[tokio::test]
async fn client_creation(ctx: &mut StoreContext) {
    let id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());
    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Noop,
            token,
        })
        .await
        .unwrap();
    // Cleaning up records
    ctx.clients.delete_client(TENANT_ID, &id).await.unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_creation_fcm(ctx: &mut StoreContext) {
    let id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());
    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token,
        })
        .await
        .unwrap();
    // Cleaning up records
    ctx.clients.delete_client(TENANT_ID, &id).await.unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_creation_apns(ctx: &mut StoreContext) {
    let id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());
    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Apns,
            token,
        })
        .await
        .unwrap();
    // Cleaning up records
    ctx.clients.delete_client(TENANT_ID, &id).await.unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_upsert_token(ctx: &mut StoreContext) {
    let client_id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());

    // Initial Client creation
    ctx.clients
        .create_client(TENANT_ID, &client_id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: token.clone(),
        })
        .await
        .unwrap();
    let insert_result = ctx.clients.get_client(TENANT_ID, &client_id).await.unwrap();
    assert_eq!(insert_result.token, token);

    // Insert notification for the client to test the clients->notifications
    // constraint works properly
    let notification_id = format!("id-{}", gen_id());
    let notification_payload = format!("payload-{}", gen_id());

    ctx.notifications
        .create_or_update_notification(
            &notification_id,
            TENANT_ID,
            &client_id,
            &echo_server::handlers::push_message::MessagePayload {
                topic: String::new(),
                flags: 0,
                blob: notification_payload,
            },
        )
        .await
        .unwrap();
    let get_notification_result = ctx
        .notifications
        .get_notification(&notification_id, &client_id, TENANT_ID)
        .await
        .unwrap();
    assert_eq!(get_notification_result.client_id, client_id);

    // Updating token for the same id
    let updated_token = format!("token-{}", gen_id());
    ctx.clients
        .create_client(TENANT_ID, &client_id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Apns,
            token: updated_token.clone(),
        })
        .await
        .unwrap();
    let updated_token_result = ctx.clients.get_client(TENANT_ID, &client_id).await.unwrap();
    assert_eq!(updated_token_result.token, updated_token);

    // Cleaning up records
    ctx.clients
        .delete_client(TENANT_ID, &client_id)
        .await
        .unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_upsert_id(ctx: &mut StoreContext) {
    let client_id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());

    // Initial Client creation
    ctx.clients
        .create_client(TENANT_ID, &client_id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: token.clone(),
        })
        .await
        .unwrap();
    let insert_result = ctx.clients.get_client(TENANT_ID, &client_id).await.unwrap();
    assert_eq!(insert_result.token, token.clone());

    // Insert notification for the client to test the clients->notifications
    // constraint works properly
    let notification_id = format!("id-{}", gen_id());
    let notification_payload = format!("payload-{}", gen_id());

    ctx.notifications
        .create_or_update_notification(
            &notification_id,
            TENANT_ID,
            &client_id,
            &echo_server::handlers::push_message::MessagePayload {
                topic: String::new(),
                flags: 0,
                blob: notification_payload,
            },
        )
        .await
        .unwrap();
    let get_notification_result = ctx
        .notifications
        .get_notification(&notification_id, &client_id, TENANT_ID)
        .await
        .unwrap();
    assert_eq!(get_notification_result.client_id, client_id);

    // Updating id for the same token
    let updated_id = gen_id();
    ctx.clients
        .create_client(TENANT_ID, &updated_id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: token.clone(),
        })
        .await
        .unwrap();
    let updated_id_result = ctx
        .clients
        .get_client(TENANT_ID, &updated_id)
        .await
        .unwrap();
    assert_eq!(updated_id_result.token, token);

    // Cleaning up records
    ctx.clients
        .delete_client(TENANT_ID, &updated_id)
        .await
        .unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_create_same_id_and_token(ctx: &mut StoreContext) {
    let client_id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());

    // Initial Client creation
    ctx.clients
        .create_client(TENANT_ID, &client_id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: token.clone(),
        })
        .await
        .unwrap();
    let insert_result = ctx.clients.get_client(TENANT_ID, &client_id).await.unwrap();
    assert_eq!(insert_result.token, token.clone());

    // Insert notification for the client to test the clients->notifications
    // constraint works properly
    let notification_id = format!("id-{}", gen_id());
    let notification_payload = format!("payload-{}", gen_id());

    ctx.notifications
        .create_or_update_notification(
            &notification_id,
            TENANT_ID,
            &client_id,
            &echo_server::handlers::push_message::MessagePayload {
                topic: String::new(),
                flags: 0,
                blob: notification_payload,
            },
        )
        .await
        .unwrap();
    let get_notification_result = ctx
        .notifications
        .get_notification(&notification_id, &client_id, TENANT_ID)
        .await
        .unwrap();
    assert_eq!(get_notification_result.client_id, client_id);

    // Create a Client with the same id and same device token
    // changing only the push_type to check if it was upserted
    ctx.clients
        .create_client(TENANT_ID, &client_id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Noop,
            token: token.clone(),
        })
        .await
        .unwrap();
    let double_insert_result = ctx.clients.get_client(TENANT_ID, &client_id).await.unwrap();
    assert_eq!(double_insert_result.token, token);
    assert_eq!(double_insert_result.push_type, ProviderKind::Noop);

    // Cleaning up records
    ctx.clients
        .delete_client(TENANT_ID, &client_id)
        .await
        .unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_deletion(ctx: &mut StoreContext) {
    let id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());

    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Noop,
            token,
        })
        .await
        .unwrap();
    ctx.clients.delete_client(TENANT_ID, &id).await.unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_fetch(ctx: &mut StoreContext) {
    let id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());

    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Noop,
            token: token.clone(),
        })
        .await
        .unwrap();

    let client = ctx.clients.get_client(TENANT_ID, &id).await.unwrap();

    assert_eq!(client.token, token);
    assert_eq!(client.push_type, ProviderKind::Noop);

    // Cleaning up records
    ctx.clients.delete_client(TENANT_ID, &id).await.unwrap();
}
