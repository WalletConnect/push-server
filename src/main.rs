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
use crate::state::{Metrics, State};

use opentelemetry::{KeyValue};
use opentelemetry::sdk::{trace::{self, IdGenerator, Sampler}, Resource};
use opentelemetry::sdk::metrics::{selectors};
use opentelemetry::util::tokio_interval_stream;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use std::time::Duration;

use tracing::{error, info};

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

fn router(state: State) -> Router<Body, Infallible> {
    Router::builder()
        .data(state)
        .middleware(Middleware::pre(middleware::logger::middleware))
        .get("/health", handlers::health::handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() -> error::Result<()> {
    dotenv().ok();
    let config = env::get_config().expect("Failed to load config, please ensure all env vars are defined.");

    tracing_subscriber::fmt()
        .with_max_level(config.log_level())
        .init();

    let redis_client = redis::Client::open(config.redis_url.as_str())?;

    let mut state = state::new_state(config, redis_client);

    if state.config.telemetry_enabled.unwrap_or(false) {
        info!("Enabling Telemetry");
        let grpc_url = state.config.telemetry_grpc_url.clone().unwrap_or("http://localhost:4317".to_string());

        let tracing_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(grpc_url.clone())
            .with_timeout(Duration::from_secs(5))
            .with_protocol(Protocol::Grpc);

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(tracing_exporter)
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_id_generator(IdGenerator::default())
                    .with_max_events_per_span(64)
                    .with_max_attributes_per_span(16)
                    .with_max_events_per_span(16)
                    .with_resource(Resource::new(vec![KeyValue::new("service.name", "echo-server")])),
            )
            .install_batch(opentelemetry::runtime::Tokio)?;

        let metrics_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(grpc_url)
            .with_timeout(Duration::from_secs(5))
            .with_protocol(Protocol::Grpc);

        let meter_provider = opentelemetry_otlp::new_pipeline()
            .metrics(tokio::spawn, tokio_interval_stream)
            .with_exporter(metrics_exporter)
            .with_period(Duration::from_secs(3))
            .with_timeout(Duration::from_secs(10))
            .with_aggregator_selector(selectors::simple::Selector::Exact)
            .build()?;

        opentelemetry::global::set_meter_provider(meter_provider.provider());

        let meter = opentelemetry::global::meter("echo-server");
        let hooks_counter = meter
            .u64_counter("registered_webhooks")
            .with_description("The number of currently registered webhooks")
            .init();

        let notification_counter = meter
            .u64_counter("received_notifications")
            .with_description("The number of notification received")
            .init();

        state.set_telemetry(tracer, Metrics {
            registered_webhooks: hooks_counter,
            received_notifications: notification_counter
        })
    }

    let port = state.config.port;
    let build_version = state.build_info.crate_info.version.clone();

    let router = router(state);

    let service = RouterService::new(router).unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let server = Server::bind(&addr).serve(service);

    info!("echo-server v{} is running at: {}", build_version, addr);
    if let Err(err) = server.await {
        opentelemetry::global::shutdown_tracer_provider();
        error!("Server error: {}", err);
    }

    Ok(())
}