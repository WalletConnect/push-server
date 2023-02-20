use {
    crate::{context::StoreContext, functional::stores::gen_id},
    echo_server::handlers::push_message::MessagePayload,
    test_context::test_context,
};

pub const TENANT_ID: &str = "000-000-000-000";
pub const CLIENT_ID: &str = "000-000-000-000";

#[test_context(StoreContext)]
#[tokio::test]
async fn notification_creation(ctx: &mut StoreContext) {
    let res = ctx
        .notifications
        .create_or_update_notification(&gen_id(), TENANT_ID, CLIENT_ID, &MessagePayload {
            topic: None,
            flags: 0,
            blob: "example-payload".to_string(),
        })
        .await;

    assert!(res.is_ok())
}
