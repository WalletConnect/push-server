use {
    crate::{
        context::StoreContext,
        functional::stores::{gen_id, TENANT_ID},
    },
    echo_server::{providers::ProviderKind, stores::client::Client},
    test_context::test_context,
};

pub const TOKEN: &str = "noop-111-222-333";

#[test_context(StoreContext)]
#[tokio::test]
async fn client_creation(ctx: &mut StoreContext) {
    let res = ctx
        .clients
        .create_client(TENANT_ID, &gen_id(), Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Noop,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok())
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_creation_fcm(ctx: &mut StoreContext) {
    let res = ctx
        .clients
        .create_client(TENANT_ID, &gen_id(), Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok())
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_creation_apns(ctx: &mut StoreContext) {
    let res = ctx
        .clients
        .create_client(TENANT_ID, &gen_id(), Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Apns,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok())
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_upsert_token(ctx: &mut StoreContext) {
    let id = gen_id();

    // Initial Client creation
    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: TOKEN.to_string(),
        })
        .await
        .unwrap();
    let insert_result = ctx.clients.get_client(TENANT_ID, &id).await.unwrap();
    assert_eq!(insert_result.token, TOKEN);

    // Updating token for the same id
    let updated_token = gen_id();
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
    let id = gen_id();

    // Initial Client creation
    ctx.clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: TOKEN.to_string(),
        })
        .await
        .unwrap();
    let insert_result = ctx.clients.get_client(TENANT_ID, &id).await.unwrap();
    assert_eq!(insert_result.token, TOKEN);

    // Updating id for the same token
    let updated_id = gen_id();
    ctx.clients
        .create_client(TENANT_ID, &updated_id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Fcm,
            token: TOKEN.to_string(),
        })
        .await
        .unwrap();
    let updated_id_result = ctx
        .clients
        .get_client(TENANT_ID, &updated_id)
        .await
        .unwrap();
    assert_eq!(updated_id_result.token, TOKEN);

    // Cleaning up records
    ctx.clients
        .delete_client(TENANT_ID, &updated_id)
        .await
        .unwrap();
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_deletion(ctx: &mut StoreContext) {
    let id = gen_id();

    let res = ctx
        .clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Noop,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok());

    let delete_res = ctx.clients.delete_client(TENANT_ID, &id).await;

    assert!(delete_res.is_ok());
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_fetch(ctx: &mut StoreContext) {
    let id = gen_id();

    let res = ctx
        .clients
        .create_client(TENANT_ID, &id, Client {
            tenant_id: TENANT_ID.to_string(),
            push_type: ProviderKind::Noop,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok());

    let client_res = ctx.clients.get_client(TENANT_ID, &id).await;

    assert!(client_res.is_ok());

    let client = client_res.expect("failed to unwrap client");

    assert_eq!(client.token, TOKEN.to_string());
    assert_eq!(client.push_type, ProviderKind::Noop);
}
