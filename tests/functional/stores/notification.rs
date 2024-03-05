use {
    crate::{
        context::StoreContext,
        functional::stores::{gen_id, TENANT_ID},
    },
    echo_server::{
        handlers::push_message::PushMessageBody, providers::ProviderKind, state::ClientStoreArc,
        stores::client::Client,
    },
    test_context::test_context,
};

pub async fn create_client(client_store: &ClientStoreArc) -> String {
    let id = format!("id-{}", gen_id());
    let token = format!("token-{}", gen_id());

    client_store
        .create_client(
            TENANT_ID,
            &id,
            Client {
                tenant_id: TENANT_ID.to_string(),
                push_type: ProviderKind::Noop,
                token,
                always_raw: false,
            },
        )
        .await
        .expect("failed to create client for notification test");

    id
}

#[test_context(StoreContext)]
#[tokio::test]
async fn notification(ctx: &mut StoreContext) {
    let client_id = create_client(&ctx.clients).await;

    let res = ctx
        .notifications
        .create_or_update_notification(
            &gen_id(),
            TENANT_ID,
            &client_id,
            &PushMessageBody {
                raw: None,
                legacy: None,
            },
        )
        .await;

    assert!(res.is_ok());
}

#[test_context(StoreContext)]
#[tokio::test]
async fn notification_multiple_clients_same_payload(ctx: &mut StoreContext) {
    let message_id = gen_id();
    let payload = PushMessageBody {
        raw: None,
        legacy: None,
    };

    let client_id1 = create_client(&ctx.clients).await;
    let res = ctx
        .notifications
        .create_or_update_notification(&message_id, TENANT_ID, &client_id1, &payload)
        .await;
    assert!(res.is_ok());

    let client_id2 = create_client(&ctx.clients).await;
    let res = ctx
        .notifications
        .create_or_update_notification(&message_id, TENANT_ID, &client_id2, &payload)
        .await;
    assert!(res.is_ok());

    let notification1 = ctx
        .notifications
        .get_notification(&message_id, &client_id1, TENANT_ID)
        .await
        .unwrap();
    assert_eq!(notification1.client_id, client_id1);

    let notification2 = ctx
        .notifications
        .get_notification(&message_id, &client_id2, TENANT_ID)
        .await
        .unwrap();
    assert_eq!(notification2.client_id, client_id2);
}
