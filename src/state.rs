use crate::providers::Providers;
use crate::relay::RelayClient;
use crate::stores::client::ClientStore;
use crate::stores::notification::NotificationStore;
use crate::{env::Config, providers::ProviderKind};
use build_info::BuildInfo;
use opentelemetry::metrics::{Counter, UpDownCounter};
use opentelemetry::sdk::trace::Tracer;
use std::sync::Arc;
use tracing_subscriber::prelude::*;

#[derive(Clone)]
pub struct Metrics {
    pub registered_webhooks: UpDownCounter<i64>,
    pub received_notifications: Counter<u64>,
}

pub trait State<C, N>
where
    C: ClientStore,
    N: NotificationStore,
{
    fn config(&self) -> Config;
    fn build_info(&self) -> BuildInfo;
    fn client_store(&self) -> C;
    fn notification_store(&self) -> N;
    fn providers(&self) -> Providers;
    fn supported_providers(&self) -> Vec<ProviderKind>;
    fn relay_client(&self) -> RelayClient;
}

#[derive(Clone)]
pub struct AppState<C, N>
where
    C: ClientStore,
    N: NotificationStore,
{
    pub config: Config,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,
    pub client_store: C,
    pub notification_store: N,
    pub providers: Providers,
    pub supported_providers: Vec<ProviderKind>,
    pub relay_client: RelayClient,
}

build_info::build_info!(fn build_info);

pub fn new_state<C, N>(
    config: Config,
    client_store: C,
    notification_store: N,
) -> crate::error::Result<AppState<C, N>>
where
    C: ClientStore,
    N: NotificationStore,
{
    let build_info: &BuildInfo = build_info();
    let providers = Providers::new(&config)?;
    let supported_providers = config.supported_providers();

    Ok(AppState {
        config,
        build_info: build_info.clone(),
        metrics: None,
        client_store,
        notification_store,
        providers,
        supported_providers,
        relay_client: RelayClient::new("https://relay.walletconnect.com".to_string()),
    })
}

impl<C, N> AppState<C, N>
where
    C: ClientStore,
    N: NotificationStore,
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

impl<C, N> State<C, N> for Arc<AppState<C, N>>
where
    C: Clone + ClientStore,
    N: Clone + NotificationStore,
{
    fn config(&self) -> Config {
        self.config.clone()
    }

    fn build_info(&self) -> BuildInfo {
        self.build_info.clone()
    }

    fn client_store(&self) -> C {
        self.client_store.clone()
    }

    fn notification_store(&self) -> N {
        self.notification_store.clone()
    }

    fn providers(&self) -> Providers {
        self.providers.clone()
    }

    fn supported_providers(&self) -> Vec<ProviderKind> {
        self.supported_providers.clone()
    }

    fn relay_client(&self) -> RelayClient {
        self.relay_client.clone()
    }
}
