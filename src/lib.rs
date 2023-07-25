use tower_http::catch_panic::CatchPanicLayer;
use {
    crate::{
        log::prelude::*,
        request_id::{GenericRequestId, X_REQUEST_ID},
        state::TenantStoreArc,
    },
    axum::{
        routing::{delete, get, post},
        Router,
    },
    config::Config,
    opentelemetry::{sdk::Resource, KeyValue},
    sqlx::{
        postgres::{PgConnectOptions, PgPoolOptions},
        ConnectOptions,
    },
    std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration},
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
    tower_http::{
        request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
        trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    },
    tracing::{info, log::LevelFilter, warn, Level},
};
#[cfg(feature = "multitenant")]
use {
    hyper::http::Method,
    tower_http::cors::{AllowOrigin, CorsLayer},
};

#[cfg(not(feature = "multitenant"))]
use crate::stores::tenant::DefaultTenantStore;

#[cfg(feature = "analytics")]
pub mod analytics;
#[cfg(not(feature = "analytics"))]
pub mod analytics {
    pub mod message_info {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct MessageInfo;
    }
}
pub mod blob;
pub mod config;
pub mod error;
pub mod handlers;
pub mod log;
pub mod macros;
pub mod metrics;
pub mod middleware;
pub mod networking;
pub mod providers;
pub mod relay;
pub mod request_id;
pub mod state;
pub mod stores;
pub mod supabase;

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

    #[cfg(not(feature = "multitenant"))]
    let tenant_store: TenantStoreArc = Arc::new(DefaultTenantStore::new(Arc::new(config.clone()))?);

    #[cfg(feature = "multitenant")]
    let tenant_store: TenantStoreArc = {
        let tenant_pg_options = PgConnectOptions::from_str(&config.tenant_database_url)?
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

        Arc::new(tenant_database)
    };

    let mut state = state::new_state(
        config,
        Arc::new(store.clone()),
        Arc::new(store.clone()),
        tenant_store,
    )?;

    #[cfg(feature = "analytics")]
    {
        if let Some(ip) = state.public_ip {
            let analytics = analytics::initialize(&state.config, ip).await?;
            state.analytics = Some(analytics);
        }
    }

    #[cfg(feature = "multitenant")]
    let supported_providers_string = "multi-tenant".to_string();

    #[cfg(not(feature = "multitenant"))]
    let supported_providers_string = state
        .config
        .single_tenant_supported_providers()
        .into_iter()
        .map(Into::into)
        .collect::<Vec<&str>>()
        .join(", ");

    // Fetch public key so it's cached for the first 6hrs
    let public_key = state.relay_client.public_key().await;
    if public_key.is_err() {
        warn!("Failed initial fetch of Relay's Public Key, this may prevent webhook validation.")
    }

    if state.config.telemetry_prometheus_port.is_some() {
        state.set_metrics(metrics::Metrics::new(Resource::new(vec![
            KeyValue::new("service_name", "echo-server"),
            KeyValue::new(
                "service_version",
                state.build_info.crate_info.version.clone().to_string(),
            ),
        ]))?);
    }

    let port = state.config.port;
    let private_port = state.config.telemetry_prometheus_port.unwrap_or(3001);
    let build_version = state.build_info.crate_info.version.clone();
    let build_commit = match state.build_info.version_control.clone() {
        Some(v) => v.git().unwrap().commit_short_id.clone(),
        None => "unknown-commit".to_string(),
    };
    let build_rustc_version = state.build_info.compiler.version.clone();
    let show_header = !state.config.disable_header;
    // TODO use value again
    let _allowed_origins = state.config.cors_allowed_origins.clone();

    let state_arc = Arc::new(state);

    let global_middleware = ServiceBuilder::new()
        // set `x-request-id` header on all requests
        .layer(SetRequestIdLayer::new(
            X_REQUEST_ID.clone(),
            GenericRequestId,
        ))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::DEBUG)
                        .include_headers(true),
                ),
        )
        .layer(PropagateRequestIdLayer::new(X_REQUEST_ID.clone()))
        .layer(CatchPanicLayer::new());

    #[cfg(feature = "multitenant")]
    let app = {
        let tenancy_routes = Router::new()
            .route("/", post(handlers::create_tenant::handler))
            .route(
                "/:id",
                get(handlers::get_tenant::handler).delete(handlers::delete_tenant::handler),
            )
            .route("/:id/fcm", post(handlers::update_fcm::handler))
            .route("/:id/apns", post(handlers::update_apns::handler))
            .layer(
                global_middleware.clone().layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::DELETE])
                        /*
                            TODO: switch back to using the real configuration
                            allowed_origins
                                .iter()
                                .map(|v| v.parse::<HeaderValue>().unwrap())
                                .collect::<Vec<HeaderValue>>(), */
                        .allow_origin(AllowOrigin::any())
                        .allow_headers([hyper::http::header::CONTENT_TYPE, hyper::http::header::AUTHORIZATION]),
                ),
            );

        Router::new()
            .route("/health", get(handlers::health::handler))
            .route("/info", get(handlers::info::handler))
            .nest("/tenants", tenancy_routes)
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
            .with_state(state_arc.clone())
    };

    #[cfg(not(feature = "multitenant"))]
    let app = Router::new()
        .route("/health", get(handlers::health::handler))
        .route("/info", get(handlers::info::handler))
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
        .layer(global_middleware)
        .with_state(state_arc.clone());

    let private_app = Router::new()
        .route("/metrics", get(handlers::metrics::handler))
        .with_state(state_arc);

    if show_header {
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

        println!("{header}");
    } else {
        debug!("Online and listening at http://0.0.0.0:{}", port.clone())
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let private_addr = SocketAddr::from(([0, 0, 0, 0], private_port));

    select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service_with_connect_info::<SocketAddr>()) => info!("Server terminating"),
        _ = axum::Server::bind(&private_addr).serve(private_app.into_make_service()) => info!("Internal Server terminating"),
        _ = shutdown.recv() => info!("Shutdown signal received, killing servers"),
    }

    Ok(())
}
