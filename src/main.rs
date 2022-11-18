use dotenv::dotenv;
use echo_server::error::Result;
use echo_server::state::TenantStoreArc;
use echo_server::stores::tenant::DefaultTenantStore;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::log::LevelFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = echo_server::env::get_config()
        .expect("Failed to load config, please ensure all env vars are defined.");

    // Check config is valid and then throw the error if its not
    config.is_valid()?;

    let pg_options = PgConnectOptions::from_str(&config.database_url)?
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Info, Duration::from_millis(250))
        .clone();

    let store = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(pg_options)
        .await?;

    // Run database migrations. `./migrations` is the path to migrations, relative to the root dir (the directory
    // containing `Cargo.toml`).
    sqlx::migrate!("./migrations").run(&store).await?;

    let mut tenant_store: TenantStoreArc =
        Arc::new(DefaultTenantStore::new(Arc::new(config.clone()))?);
    if let Some(tenant_database_url) = &config.tenant_database_url {
        let tenant_pg_options = PgConnectOptions::from_str(tenant_database_url)?
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Info, Duration::from_millis(250))
            .clone();

        let tenant_database = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(tenant_pg_options)
            .await?;

        // Run database migrations. `./tenant_migrations` is the path to migrations, relative to the root dir (the directory
        // containing `Cargo.toml`).
        sqlx::migrate!("./tenant_migrations")
            .run(&tenant_database)
            .await?;

        tenant_store = Arc::new(tenant_database);
    }

    let state = echo_server::state::new_state(
        config,
        Arc::new(store.clone()),
        Arc::new(store.clone()),
        tenant_store,
    )?;

    Ok(echo_server::bootstap(state).await?)
}
