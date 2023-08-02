/// Tests against the handlers
use {crate::context::EchoServerContext, test_context::test_context};

mod apns;
mod fcm;
mod tenancy;

#[test_context(EchoServerContext)]
#[tokio::test]
async fn test_health(ctx: &mut EchoServerContext) {
    let body = reqwest::get(format!("http://{}/health", ctx.server.public_addr))
        .await
        .expect("Failed to call /health")
        .status();
    assert!(body.is_success());
}
