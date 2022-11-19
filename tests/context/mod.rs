use self::server::EchoServer;

use {async_trait::async_trait, test_context::AsyncTestContext};

mod server;

pub struct ServerContext {
    pub server: EchoServer,
}

#[async_trait]
impl AsyncTestContext for ServerContext {
    async fn setup() -> Self {
        let server = EchoServer::start().await;
        Self { server }
    }

    async fn teardown(mut self) {
        self.server.shutdown().await;
    }
}
