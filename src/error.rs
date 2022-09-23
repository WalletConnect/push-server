use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Envy(envy::Error),
    Trace(opentelemetry::trace::TraceError),
    Metrics(opentelemetry::metrics::MetricsError),
    Apns(a2::Error),
    Fcm(fcm::FcmError),
    Io(std::io::Error),
    ProviderNotFound(String),
    ProviderNotAvailable(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Envy(err) => write!(f, "{}", err),
            Error::Trace(err) => Debug::fmt(&err, f),
            Error::Metrics(err) => Debug::fmt(&err, f),
            Error::Apns(err) => Debug::fmt(&err, f),
            Error::Fcm(err) => write!(f, "{}", err),
            Error::Io(err) => write!(f, "{}", err),
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
        Error::Envy(err)
    }
}

impl From<opentelemetry::trace::TraceError> for Error {
    fn from(err: opentelemetry::trace::TraceError) -> Self {
        Error::Trace(err)
    }
}

impl From<opentelemetry::metrics::MetricsError> for Error {
    fn from(err: opentelemetry::metrics::MetricsError) -> Self {
        Error::Metrics(err)
    }
}

impl From<a2::Error> for Error {
    fn from(err: a2::Error) -> Self {
        Error::Apns(err)
    }
}

impl From<fcm::FcmError> for Error {
    fn from(err: fcm::FcmError) -> Self {
        Error::Fcm(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::error::Error for Error {}
