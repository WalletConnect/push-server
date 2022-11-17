use axum::{
    routing::{delete, get, post},
    Router,
};
use opentelemetry::sdk::metrics::selectors;
use opentelemetry::sdk::{
    trace::{self, IdGenerator, Sampler},
    Resource,
};
use opentelemetry::util::tokio_interval_stream;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use state::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{warn, Level};

use crate::state::Metrics;
use tracing_subscriber::fmt::format::FmtSpan;

pub mod env;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod providers;
pub mod relay;
pub mod state;
pub mod stores;

pub async fn bootstap(
    mut state: AppState,
    supported_providers_string: String,
) -> error::Result<()> {
    // Fetch public key so it's cached for the first 6hrs
    let public_key = state.relay_client.public_key().await;
    if public_key.is_err() {
        warn!("Failed initial fetch of Relay's Public Key, this may prevent webhook validation.")
    }

    if state.config.telemetry_enabled.unwrap_or(false) {
        let grpc_url = state
            .config
            .telemetry_grpc_url
            .clone()
            .unwrap_or_else(|| "http://localhost:4317".to_string());

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
                    .with_resource(Resource::new(vec![
                        KeyValue::new("service.name", "echo-server"),
                        KeyValue::new(
                            "service.version",
                            state.build_info.crate_info.version.clone().to_string(),
                        ),
                    ])),
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
            .i64_up_down_counter("registered_webhooks")
            .with_description("The number of currently registered webhooks")
            .init();

        let notification_counter = meter
            .u64_counter("received_notifications")
            .with_description("The number of notification received")
            .init();

        state.set_telemetry(
            tracer,
            Metrics {
                registered_webhooks: hooks_counter,
                received_notifications: notification_counter,
            },
        )
    } else {
        // Only log to console if telemetry disabled
        tracing_subscriber::fmt()
            .with_max_level(state.config.log_level())
            .with_span_events(FmtSpan::CLOSE)
            .init();
    }

    let port = state.config.port;
    let build_version = state.build_info.crate_info.version.clone();
    let build_commit = state
        .build_info
        .version_control
        .clone()
        .unwrap()
        .git()
        .unwrap()
        .commit_short_id
        .clone();
    let build_rustc_version = state.build_info.compiler.version.clone();

    let state_arc = Arc::new(state);

    let global_middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .include_headers(true),
            ),
    );

    let app = Router::with_state(state_arc)
        .route("/health", get(handlers::health::handler))
        .route(
            "/clients",
            post(handlers::single_tenant_wrappers::register_handler),
        )
        .route(
            "/clients/:id",
            delete(handlers::single_tenant_wrappers::delete_handler),
        )
        .route(
            "/clients/:id",
            post(handlers::single_tenant_wrappers::push_handler),
        )
        .route(
            "/:tenant_id/clients",
            post(handlers::register_client::handler),
        )
        .route(
            "/:tenant_id/clients/:id",
            delete(handlers::delete_client::handler),
        )
        .route(
            "/:tenant_id/clients/:id",
            post(handlers::push_message::handler),
        )
        .layer(global_middleware);

    let header = format!(
        "
 ______       _               _____
|  ____|     | |             / ____|
| |__    ___ | |__    ___   | (___    ___  _ __ __   __ ___  _ __
|  __|  / __|| '_ \\  / _ \\   \\___ \\  / _ \\| '__|\\ \\ / // _ \\| '__|
| |____| (__ | | | || (_) |  ____) ||  __/| |    \\ V /|  __/| |
|______|\\___||_| |_| \\___/  |_____/  \\___||_|     \\_/  \\___||_|
\nversion: {}, commit: {}, rustc: {},
web-host: {}, web-port: {},
providers: [{}]
",
        build_version,
        build_commit,
        build_rustc_version,
        "0.0.0.0",
        port.clone(),
        supported_providers_string
    );
    println!("{}", header);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
