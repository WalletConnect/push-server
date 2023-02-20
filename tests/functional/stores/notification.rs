
use {
    crate::context::StoreContext,
    test_context::test_context,
    uuid::Uuid,
};
use echo_server::handlers::push_message::MessagePayload;

pub const TENANT_ID: &str = "000-000-000-000";
pub const CLIENT_ID: &str = "000-000-000-000";

pub fn gen_id() -> &str {
    &Uuid::new_v4().to_string()
}

#[test_context(StoreContext)]
#[tokio::test]
async fn notification_creation(ctx: &mut StoreContext) {
    let res = ctx
        .notifications
        .create_or_update_notification(gen_id(), TENANT_ID, CLIENT_ID, &MessagePayload {
            topic: None,
            flags: 0,
            blob: "example-payload".to_string(),
        })
        .await;

    assert!(res.is_ok())
}