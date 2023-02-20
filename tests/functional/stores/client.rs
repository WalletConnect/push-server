use {
    crate::context::StoreContext,
    echo_server::{providers::ProviderKind, stores::client::Client},
    uuid::Uuid,
};

pub const TENANT_ID: &str = "000-000-000-000";
pub const TOKEN: &str = "noop-111-222-333";

pub fn gen_id() -> &str {
    &Uuid::new_v4().to_string()
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_creation(ctx: &mut StoreContext) {
    let res = ctx
        .clients
        .create_client(TENANT_ID, gen_id(), Client {
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
        .create_client(TENANT_ID, gen_id(), Client {
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
        .create_client(TENANT_ID, gen_id(), Client {
            push_type: ProviderKind::Apns,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok())
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_upsert(ctx: &mut StoreContext) {
    let id = gen_id();

    let res = ctx
        .clients
        .create_client(TENANT_ID, id, Client {
            push_type: ProviderKind::Apns,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok());

    let upsert_res = ctx
        .clients
        .create_client(TENANT_ID, id, Client {
            push_type: ProviderKind::Fcm,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(upsert_res.is_ok());
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_deletion(ctx: &mut StoreContext) {
    let id = gen_id();

    let res = ctx
        .clients
        .create_client(TENANT_ID, id, Client {
            push_type: ProviderKind::Noop,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok());

    let delete_res = ctx.clients.delete_client(TENANT_ID, id).await;

    assert!(delete_res.is_ok());
}

#[test_context(StoreContext)]
#[tokio::test]
async fn client_fetch(ctx: &mut StoreContext) {
    let id = gen_id();

    let res = ctx
        .clients
        .create_client(TENANT_ID, id, Client {
            push_type: ProviderKind::Noop,
            token: TOKEN.to_string(),
        })
        .await;

    assert!(res.is_ok());

    let client_res = ctx.clients.get_client(TENANT_ID, id).await;

    assert!(client_res.is_ok());

    let client = client_res.expect("failed to unwrap client");

    assert_eq!(client.token, TOKEN.to_string());
    assert_eq!(client.push_type, ProviderKind::Noop);
}