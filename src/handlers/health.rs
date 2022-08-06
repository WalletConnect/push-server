use routerify::prelude::*;

use std::convert::Infallible;
use hyper::{Body, Request, Response};
use crate::State;

pub async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    let state = req.data::<State>().unwrap();

    *response.body_mut() = Body::from(format!("OK, echo-server v{}", state.build_info.crate_info.version));

    Ok(response)
}