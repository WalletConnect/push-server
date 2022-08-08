use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EnvyError(envy::Error),
    RedisError(redis::RedisError)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EnvyError(err) => write!(f, "{}", err),
            Error::RedisError(err) => write!(f, "{}", err),
        }
    }
}

impl From<envy::Error> for Error {
    fn from(err: envy::Error) -> Self {
        Error::EnvyError(err)
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::RedisError(err)
    }
}

impl std::error::Error for Error {}