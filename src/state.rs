use crate::providers::Providers;
use crate::store::ClientStore;
use crate::{BuildInfo, Config};
use opentelemetry::metrics::{Counter, UpDownCounter};
use opentelemetry::sdk::trace::Tracer;
use std::sync::Mutex;
use tracing_subscriber::prelude::*;

pub struct Metrics {
    pub registered_webhooks: UpDownCounter<i64>,
    pub received_notifications: Counter<u64>,
}

pub struct State<S>
where
    S: ClientStore,
{
    pub config: Config,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,
    pub store: Mutex<S>,
    pub providers: Providers,
}

build_info::build_info!(fn build_info);

pub fn new_state<S>(config: Config, store: S) -> crate::error::Result<State<S>>
where
    S: ClientStore,
{
    let build_info: &BuildInfo = build_info();
    let providers = Providers::new(&config)?;

    Ok(State {
        config,
        build_info: build_info.clone(),
        metrics: None,
        store: Mutex::new(store),
        providers,
    })
}

impl<S> State<S>
where
    S: ClientStore,
{
    pub fn set_telemetry(&mut self, tracer: Tracer, metrics: Metrics) {
        let otel_tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(otel_tracing_layer)
            .init();

        self.metrics = Some(metrics);
    }
}
