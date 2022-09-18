use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EnvyError(envy::Error),
    TraceError(opentelemetry::trace::TraceError),
    MetricsError(opentelemetry::metrics::MetricsError),
    ApnsError(a2::Error),
    FcmError(fcm::FcmError),
    IoError(std::io::Error),
    ProviderNotFound(String),
    ProviderNotAvailable(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EnvyError(err) => write!(f, "{}", err),
            Error::TraceError(err) => Debug::fmt(&err, f),
            Error::MetricsError(err) => Debug::fmt(&err, f),
            Error::ApnsError(err) => Debug::fmt(&err, f),
            Error::FcmError(err) => write!(f, "{}", err),
            Error::IoError(err) => write!(f, "{}", err),
            Error::ProviderNotFound(name) => write!(
                f,
                "{} is an invalid push provider as it cannot be not found",
                name
            ),
            Error::ProviderNotAvailable(name) => write!(
                f,
                "{} is an invalid push provider as it has not been enabled",
                name
            ),
        }
    }
}

impl From<envy::Error> for Error {
    fn from(err: envy::Error) -> Self {
        Error::EnvyError(err)
    }
}

impl From<opentelemetry::trace::TraceError> for Error {
    fn from(err: opentelemetry::trace::TraceError) -> Self {
        Error::TraceError(err)
    }
}

impl From<opentelemetry::metrics::MetricsError> for Error {
    fn from(err: opentelemetry::metrics::MetricsError) -> Self {
        Error::MetricsError(err)
    }
}

impl From<a2::Error> for Error {
    fn from(err: a2::Error) -> Self {
        Error::ApnsError(err)
    }
}

impl From<fcm::FcmError> for Error {
    fn from(err: fcm::FcmError) -> Self {
        Error::FcmError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl std::error::Error for Error {}
