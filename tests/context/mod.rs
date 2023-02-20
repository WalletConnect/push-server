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
    pub pool: Arc<Pool<Postgres>>,
    pub tenant_pool: Arc<Pool<Postgres>>,

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
        let (db, tenant_db) = stores::open_pg_connections().await;

        let db_arc = Arc::new(db);
        let tenant_db_arc = Arc::new(tenant_db);

        Self {
            pool: db_arc.clone(),
            tenant_pool: tenant_db_arc.clone(),
            clients: db_arc.clone(),
            notifications: db_arc.clone(),
            tenants: tenant_db_arc.clone(),
        }
    }

    async fn teardown(self) {
        self.pool.close().await;
        self.tenant_pool.close().await;
    }
}
