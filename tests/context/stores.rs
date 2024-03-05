use {
    sqlx::{
        postgres::{PgConnectOptions, PgPoolOptions},
        ConnectOptions, Pool, Postgres,
    },
    std::{str::FromStr, time::Duration},
    tracing::log::LevelFilter,
};

pub async fn open_pg_connections(
    database_uri: &str,
    tenant_database_uri: &str,
) -> (Pool<Postgres>, Pool<Postgres>) {
    let pg_options = PgConnectOptions::from_str(database_uri)
        .expect("failed to parse postgres url")
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Info, Duration::from_millis(250))
        .clone();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(pg_options)
        .await
        .expect("failed to connect to postgres");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let tenant_pg_options = PgConnectOptions::from_str(tenant_database_uri)
        .expect("failed to parse postgres url")
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Info, Duration::from_millis(250))
        .clone();

    let tenant_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(tenant_pg_options)
        .await
        .expect("failed to connect to postgres");

    sqlx::migrate!("./tenant_migrations")
        .run(&tenant_pool)
        .await
        .expect("failed to run migrations");

    (pool, tenant_pool)
}
