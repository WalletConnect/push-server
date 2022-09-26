pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Envy(#[from] envy::Error),

    #[error(transparent)]
    Trace(#[from] opentelemetry::trace::TraceError),

    #[error(transparent)]
    Metrics(#[from] opentelemetry::metrics::MetricsError),

    #[error(transparent)]
    Apns(#[from] a2::Error),

    #[error(transparent)]
    Fcm(#[from] fcm::FcmError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error("database migration failed: {0}")]
    DatabaseMigration(#[from] sqlx::migrate::MigrateError),

    #[error("{0} is an invalid push provider as it cannot be not found")]
    ProviderNotFound(String),

    #[error("{0} is an invalid push provider as it has not been enabled")]
    ProviderNotAvailable(String),
}
