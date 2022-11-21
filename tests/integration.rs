//mod env;
//mod providers;
//mod store; // Comment this out for now
mod context;
mod functional;

pub type ErrorResult<T> = Result<T, TestError>;

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error(transparent)]
    Elapsed(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    EchoServer(#[from] echo_server::error::Error)
}