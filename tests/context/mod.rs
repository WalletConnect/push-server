use {
    self::server::{MultiTenantEchoServer, SingleTenantEchoServer},
    async_trait::async_trait,
    echo_server::state::{ClientStoreArc, NotificationStoreArc, TenantStoreArc},
    sqlx::{Pool, Postgres},
    std::sync::Arc,
    test_context::AsyncTestContext,
};

mod server;
mod stores;

pub const DATABASE_URL: &str = "postgres://postgres:root@localhost:5432/postgres";
pub const TENANT_DATABASE_URL: &str = "postgres://postgres:root@localhost:5433/postgres";

pub struct SingleTenantServerContext {
    pub server: SingleTenantEchoServer,
}

pub struct MultiTenantServerContext {
    pub server: MultiTenantEchoServer,
}

pub struct StoreContext {
    pub pool: Pool<Postgres>,
    pub tenant_pool: Pool<Postgres>,

    pub clients: ClientStoreArc,
    pub notifications: NotificationStoreArc,
    pub tenants: TenantStoreArc,
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

#[async_trait]
impl AsyncTestContext for StoreContext {
    async fn setup() -> Self {
        let db = stores::open_pg_connection(DATABASE_URL).await;
        let tenant_db = stores::open_pg_connection(TENANT_DATABASE_URL).await;

        Self {
            pool: db,
            tenant_pool: tenant_db,
            clients: Arc::new(db.clone()),
            notifications: Arc::new(db.clone()),
            tenants: Arc::new(tenant_db.clone()),
        }
    }

    async fn teardown(self) {
        self.pool.close().await;
        self.tenant_pool.close().await;
    }
}
