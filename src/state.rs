use crate::relay::RelayClient;
use crate::stores::client::ClientStore;
use crate::stores::notification::NotificationStore;
use crate::stores::tenant::TenantStore;
use crate::{env::Config, providers::ProviderKind};
use build_info::BuildInfo;
use opentelemetry::metrics::{Counter, UpDownCounter};
use opentelemetry::sdk::trace::Tracer;
use std::sync::Arc;
use tracing_subscriber::prelude::*;

pub type ClientStoreArc = Arc<dyn ClientStore + Send + Sync + 'static>;
pub type NotificationStoreArc = Arc<dyn NotificationStore + Send + Sync + 'static>;
pub type TenantStoreArc = Arc<dyn TenantStore + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Metrics {
    pub registered_webhooks: UpDownCounter<i64>,
    pub received_notifications: Counter<u64>,
}

pub trait State {
    fn config(&self) -> Config;
    fn build_info(&self) -> BuildInfo;
    fn client_store(&self) -> ClientStoreArc;
    fn notification_store(&self) -> NotificationStoreArc;
    fn tenant_store(&self) -> TenantStoreArc;
    fn relay_client(&self) -> RelayClient;
    fn is_multitenant(&self) -> bool;
}

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,
    pub client_store: ClientStoreArc,
    pub notification_store: NotificationStoreArc,
    pub tenant_store: TenantStoreArc,
    pub relay_client: RelayClient,
    is_multitenant: bool,
}

build_info::build_info!(fn build_info);

pub fn new_state(
    config: Config,
    client_store: ClientStoreArc,
    notification_store: NotificationStoreArc,
    tenant_store: TenantStoreArc,
) -> crate::error::Result<AppState> {
    let build_info: &BuildInfo = build_info();

    Ok(AppState {
        config,
        build_info: build_info.clone(),
        metrics: None,
        client_store,
        notification_store,
        tenant_store,
        relay_client: RelayClient::new("https://relay.walletconnect.com".to_string()),
        is_multitenant: config.tenant_database_url.is_some(),
    })
}

impl AppState {
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

impl State for Arc<AppState> {
    fn config(&self) -> Config {
        self.config.clone()
    }

    fn build_info(&self) -> BuildInfo {
        self.build_info.clone()
    }

    fn client_store(&self) -> ClientStoreArc {
        self.client_store.clone()
    }

    fn notification_store(&self) -> NotificationStoreArc {
        self.notification_store.clone()
    }

    fn tenant_store(&self) -> TenantStoreArc {
        self.tenant_store.clone()
    }

    fn relay_client(&self) -> RelayClient {
        self.relay_client.clone()
    }

    fn is_multitenant(&self) -> bool {
        self.is_multitenant
    }
}
