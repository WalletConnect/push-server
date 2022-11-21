use {crate::context::ServerContext, test_context::test_context};

#[test_context(ServerContext)]
#[tokio::test]
async fn test_health(ctx: &mut ServerContext) {
    let body = reqwest::get(format!("http://{}/health", ctx.server.public_addr))
        .await
        .expect("Failed to call /health")
        .status();
    assert!(body.is_success());
}
