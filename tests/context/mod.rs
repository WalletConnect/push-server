use {
    self::server::{MultiTenantEchoServer, SingleTenantEchoServer},
    async_trait::async_trait,
    test_context::AsyncTestContext,
};

mod server;

pub struct SingleTenantServerContext {
    pub server: SingleTenantEchoServer,
}

pub struct MultiTenantServerContext {
    pub server: MultiTenantEchoServer,
}

#[async_trait]
impl AsyncTestContext for SingleTenantServerContext {
    async fn setup() -> Self {
        let server = SingleTenantEchoServer::start().await;
        Self { server }
    }

    async fn teardown(mut self) {
        self.server.shutdown().await;
    }
}

#[async_trait]
impl AsyncTestContext for MultiTenantServerContext {
    async fn setup() -> Self {
        let server = MultiTenantEchoServer::start().await;
        Self { server }
    }

    async fn teardown(mut self) {
        self.server.shutdown().await;
    }
}
