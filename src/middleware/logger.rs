use routerify::prelude::*;

use std::convert::Infallible;
use hyper::{Body, Request};
use tracing::trace;

pub async fn middleware(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    trace!("{} {} {}", req.remote_addr(), req.method(), req.uri().path());
    Ok(req)
}