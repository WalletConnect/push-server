use {
    self::server::EchoServer,
    async_trait::async_trait,
    echo_server::{
        config::Config,
        state::{ClientStoreArc, NotificationStoreArc, TenantStoreArc},
    },
    sqlx::{Pool, Postgres},
    std::sync::Arc,
    test_context::{AsyncTestContext, TestContext},
};

mod server;
mod stores;

pub struct ConfigContext {
    pub config_from_env: Config,
}

pub struct EchoServerContext {
    pub server: EchoServer,
}

pub struct StoreContext {
    pub pool: Arc<Pool<Postgres>>,
    pub tenant_pool: Arc<Pool<Postgres>>,

    pub clients: ClientStoreArc,
    pub notifications: NotificationStoreArc,
    pub tenants: TenantStoreArc,
}

impl TestContext for ConfigContext {
    fn setup() -> Self {
        let config_from_env = envy::from_env::<Config>().unwrap();
        Self { config_from_env }
    }
}

#[async_trait]
impl AsyncTestContext for EchoServerContext {
    async fn setup() -> Self {
        let config_from_env = ConfigContext::setup().config_from_env;
        let server = EchoServer::start(&config_from_env).await;
        Self { server }
    }
}

#[async_trait]
impl AsyncTestContext for StoreContext {
    async fn setup() -> Self {
        let config_from_env = ConfigContext::setup().config_from_env;
        let (db, tenant_db) = stores::open_pg_connections(
            &config_from_env.database_url,
            &config_from_env.tenant_database_url,
        )
        .await;

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
