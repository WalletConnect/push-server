/// Tests against the handlers
use {crate::context::SingleTenantServerContext, test_context::test_context};

mod push;
mod registration;
mod tenancy;

#[test_context(SingleTenantServerContext)]
#[tokio::test]
async fn test_health(ctx: &mut SingleTenantServerContext) {
    let body = reqwest::get(format!("http://{}/health", ctx.server.public_addr))
        .await
        .expect("Failed to call /health")
        .status();
    assert!(body.is_success());
}
