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
    let id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());

    // Initial Client creation
    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: token.clone(),
        })
        .await
        .unwrap();
    let insert_result = ctx.clients.get_client(TENANT_ID, &id).await.unwrap();
    assert_eq!(insert_result.token, token);

    // Updating token for the same id
    let updated_token = format!("token-{}", gen_id());
    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Apns,
            token: updated_token.clone(),
        })
        .await
        .unwrap();
    let updated_token_result = ctx.clients.get_client(TENANT_ID, &id).await.unwrap();
    assert_eq!(updated_token_result.token, updated_token);

    // Cleaning up records
    ctx.clients.delete_client(TENANT_ID, &id).await.unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_upsert_id(ctx: &mut StoreContext) {
    let id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());

    // Initial Client creation
    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: token.clone(),
        })
        .await
        .unwrap();
    let insert_result = ctx.clients.get_client(TENANT_ID, &id).await.unwrap();
    assert_eq!(insert_result.token, token.clone());

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
