mod env;
mod error;
mod handlers;
mod state;

use std::sync::Arc;
use build_info::BuildInfo;
use dotenv::dotenv;
use crate::env::Config;

use crate::state::{Metrics, State};

use opentelemetry::{KeyValue};
use opentelemetry::sdk::{trace::{self, IdGenerator, Sampler}, Resource};
use opentelemetry::sdk::metrics::{selectors};
use opentelemetry::util::tokio_interval_stream;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use std::time::Duration;

use tracing::{info};
use tracing_subscriber::fmt::format::FmtSpan;

use warp::Filter;

#[tokio::main]
async fn main() -> error::Result<()> {
    dotenv().ok();
    let config = env::get_config().expect("Failed to load config, please ensure all env vars are defined.");

    tracing_subscriber::fmt()
        .with_max_level(config.log_level())
        .with_span_events(FmtSpan::CLOSE)
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

    let state_arc = Arc::new(state);
    let state_filter = warp::any().map(move || state_arc.clone());

    let health = warp::get()
        .and(warp::path!("health"))
        .and(state_filter.clone())
        .and_then(handlers::health::handler);
    let register_client = warp::post()
        .and(warp::path!("clients"))
        .and(state_filter.clone())
        .and(warp::body::json())
        .and_then(handlers::register_client::handler);
    let delete_client = warp::delete()
        .and(warp::path!("clients" / String))
        .and(state_filter.clone())
        .and_then(handlers::delete_client::handler);
    let push_to_client = warp::post()
        .and(warp::path!("clients" / String))
        .and(state_filter)
        .and(warp::body::json())
        .and_then(handlers::push_message::handler);

    let routes = warp::any()
        .and(health
            .or(register_client)
            .or(delete_client)
            .or(push_to_client))
        .with(warp::trace::request());

    info!("v{}", build_version);
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;

    Ok(())
}