use {
    crate::{metrics::Metrics, state::TenantStoreArc, stores::tenant::DefaultTenantStore},
    axum::{
        routing::{delete, get, post},
        Router,
    },
    env::Config,
    opentelemetry::{
        sdk::{
            trace::{self, IdGenerator, Sampler},
            Resource,
        },
        KeyValue,
    },
    opentelemetry_otlp::{Protocol, WithExportConfig},
    sqlx::{
        postgres::{PgConnectOptions, PgPoolOptions},
        ConnectOptions,
    },
    std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration},
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
    tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    tracing::{info, log::LevelFilter, warn, Level},
};

pub mod blob;
pub mod env;
pub mod error;
pub mod handlers;
pub mod metrics;
pub mod middleware;
pub mod providers;
pub mod relay;
pub mod state;
pub mod stores;

pub async fn bootstap(mut shutdown: broadcast::Receiver<()>, config: Config) -> error::Result<()> {
    // Check config is valid and then throw the error if its not
    config.is_valid()?;

    let pg_options = PgConnectOptions::from_str(&config.database_url)?
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Info, Duration::from_millis(250))
        .clone();

    let store = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(pg_options)
        .await?;

    // Run database migrations. `./migrations` is the path to migrations, relative
    // to the root dir (the directory containing `Cargo.toml`).
    sqlx::migrate!("./migrations").run(&store).await?;

    let mut tenant_store: TenantStoreArc =
        Arc::new(DefaultTenantStore::new(Arc::new(config.clone()))?);
    if let Some(tenant_database_url) = &config.tenant_database_url {
        let tenant_pg_options = PgConnectOptions::from_str(tenant_database_url)?
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Info, Duration::from_millis(250))
            .clone();

        let tenant_database = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(tenant_pg_options)
            .await?;

        // Run database migrations. `./tenant_migrations` is the path to migrations,
        // relative to the root dir (the directory containing `Cargo.toml`).
        sqlx::migrate!("./tenant_migrations")
            .run(&tenant_database)
            .await?;

        tenant_store = Arc::new(tenant_database);
    }

    let mut state = state::new_state(
        config,
        Arc::new(store.clone()),
        Arc::new(store.clone()),
        tenant_store,
    )?;

    let mut supported_providers_string = "multi-tenant".to_string();
    if state.config.tenant_database_url.is_none() {
        supported_providers_string = state
            .config
            .single_tenant_supported_providers()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<&str>>()
            .join(", ");
    }

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
            .with_endpoint(grpc_url)
            .with_timeout(Duration::from_secs(5))
            .with_protocol(Protocol::Grpc);

        let resource = Resource::new(vec![
            KeyValue::new("service.name", "echo-server"),
            KeyValue::new(
                "service.version",
                state.build_info.crate_info.version.clone().to_string(),
            ),
        ]);

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
                    .with_resource(resource.clone()),
            )
            .install_batch(opentelemetry::runtime::Tokio)?;

        let metrics = Metrics::new(resource)?;

        state.set_telemetry(tracer, metrics)
    } else if !state.config.is_test && !state.config.telemetry_enabled.unwrap_or(false) {
        // Only log to console if telemetry disabled and its not in tests
        tracing_subscriber::fmt()
            .with_max_level(state.config.log_level())
            .init();
    }

    let port = state.config.port;
    let private_port = state.config.telemetry_prometheus_port;
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

    let app = Router::new()
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
        .layer(global_middleware)
        .with_state(state_arc.clone());

    let private_app = Router::new()
        .route("/metrics", get(handlers::metrics::handler))
        .with_state(state_arc);

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
    let private_addr = SocketAddr::from(([0, 0, 0, 0], private_port));

    select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service()) => info!("Server terminating"),
        _ = axum::Server::bind(&private_addr).serve(private_app.into_make_service()) => info!("Internal Server terminating"),
        _ = shutdown.recv() => info!("Shutdown signal received, killing servers"),
    }

    Ok(())
}
