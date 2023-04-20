use {
    crate::{
        context::StoreContext,
        functional::stores::{client::TOKEN, gen_id, TENANT_ID},
    },
    echo_server::{
        handlers::push_message::MessagePayload,
        providers::ProviderKind,
        state::ClientStoreArc,
        stores::client::Client,
    },
    test_context::test_context,
};

pub async fn get_client(client_store: &ClientStoreArc) -> String {
    let id = gen_id();

    client_store
        .create_client(TENANT_ID, &id, Client {
            push_type: ProviderKind::Noop,
            token: TOKEN.to_string(),
        })
        .await
        .expect("failed to create client for notification test");

    id
}

#[test_context(StoreContext)]
#[tokio::test]
async fn notification_creation(ctx: &mut StoreContext) {
    let client_id = get_client(&ctx.clients).await;

    let res = ctx
        .notifications
        .create_or_update_notification(&gen_id(), TENANT_ID, &client_id, &MessagePayload {
            topic: None,
            flags: 0,
            blob: "example-payload".to_string(),
        })
        .await;

    assert!(res.is_ok())
}
