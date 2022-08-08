mod env;
mod error;
mod handlers;
mod state;
mod middleware;

use std::convert::Infallible;
use std::net::SocketAddr;
use build_info::BuildInfo;
use dotenv::dotenv;
use hyper::{Body, Response, Server, StatusCode};
use crate::env::Config;

use routerify::{Middleware, Router, RouterService, RequestInfo};
use crate::state::State;

build_info::build_info!(fn build_info);

// Define an error handler function which will accept the `routerify::Error`
// and the request information and generates an appropriate response.
async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    eprintln!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

fn router(state: State) -> Router<Body, Infallible> {
    // Create a router and specify the logger middleware and the handlers.
    // Here, "Middleware::pre" means we're adding a pre middleware which will be executed
    // before any route handlers.
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(state)
        .middleware(Middleware::pre(middleware::logger::middleware))
        .get("/health", handlers::health::handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = env::get_config().expect("Failed to load config, please ensure all env vars are defined.");
    let build_info: &BuildInfo = build_info();

    let redis_client = redis::Client::open(config.redis_url)?;

    let router = router(State {
        config: config.clone(),
        build_info: build_info.clone(),
        redis: redis_client
    });

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("echo-server v{} is running at: {}", build_info.crate_info.version, addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}