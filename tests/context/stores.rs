use {
    sqlx::{
        postgres::{PgConnectOptions, PgPoolOptions},
        ConnectOptions,
        Pool,
        Postgres,
    },
    std::{str::FromStr, time::Duration},
    tracing::log::LevelFilter,
};

pub async fn open_pg_connection(url: &str) -> Pool<Postgres> {
    let pg_options = PgConnectOptions::from_str(url)
        .expect("failed to parse postgres url")
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Info, Duration::from_millis(250))
        .clone();

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(pg_options)
        .await
        .expect("failed to connect to postgres")
}
