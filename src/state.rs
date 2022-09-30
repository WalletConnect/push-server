use crate::providers::Providers;
use crate::relay::RelayClient;
use crate::store::ClientStore;
use crate::{env::Config, providers::ProviderKind};
use build_info::BuildInfo;
use opentelemetry::metrics::{Counter, UpDownCounter};
use opentelemetry::sdk::trace::Tracer;
use tracing_subscriber::prelude::*;

#[derive(Clone)]
pub struct Metrics {
    pub registered_webhooks: UpDownCounter<i64>,
    pub received_notifications: Counter<u64>,
}

#[derive(Clone)]
pub struct AppState<S>
where
    S: ClientStore,
{
    pub config: Config,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,
    pub store: S,
    pub providers: Providers,
    pub supported_providers: Vec<ProviderKind>,
    pub relay_client: RelayClient,
}

build_info::build_info!(fn build_info);

pub fn new_state<S>(config: Config, store: S) -> crate::error::Result<AppState<S>>
where
    S: ClientStore,
{
    let build_info: &BuildInfo = build_info();
    let providers = Providers::new(&config)?;
    let supported_providers = config.supported_providers();

    Ok(AppState {
        config,
        build_info: build_info.clone(),
        metrics: None,
        store,
        providers,
        supported_providers,
        relay_client: RelayClient::new("https://relay.walletconnect.com".to_string()),
    })
}

impl<S> AppState<S>
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

    pub fn supported_providers(&self) -> &[ProviderKind] {
        &self.supported_providers
    }
}
