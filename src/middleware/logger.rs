use routerify::prelude::*;

use std::convert::Infallible;
use hyper::{Body, Request};

pub async fn middleware(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    println!("{} {} {}", req.remote_addr(), req.method(), req.uri().path());
    Ok(req)
}