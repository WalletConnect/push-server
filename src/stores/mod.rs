pub mod client;
pub mod notification;
pub mod tenant;

type Result<T> = std::result::Result<T, StoreError>;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    /// Not found error, params are entity name and identifier
    #[error("Cannot find {0} with specified identifier {1}")]
    NotFound(String, String),
}
