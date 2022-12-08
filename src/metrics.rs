use {
    crate::error::{Error, Result},
    opentelemetry::{
        metrics::{Counter, UpDownCounter},
        sdk::Resource,
    },
    opentelemetry_prometheus::PrometheusExporter,
    prometheus::TextEncoder,
};

#[derive(Clone)]
pub struct Metrics {
    pub prometheus_exporter: PrometheusExporter,
    pub registered_webhooks: UpDownCounter<i64>,
    pub received_notifications: Counter<u64>,
}

impl Metrics {
    pub fn new(resource: Resource) -> Result<Self> {
        let exporter = opentelemetry_prometheus::exporter()
            .with_resource(resource)
            .init();

        let provider = exporter.provider()?;

        opentelemetry::global::set_meter_provider(provider);

        let meter = opentelemetry::global::meter("echo-server");
        let hooks_counter = meter
            .i64_up_down_counter("registered_webhooks")
            .with_description("The number of currently registered webhooks")
            .init();

        let notification_counter = meter
            .u64_counter("received_notifications")
            .with_description("The number of notification received")
            .init();

        Ok(Metrics {
            prometheus_exporter: exporter,
            registered_webhooks: hooks_counter,
            received_notifications: notification_counter,
        })
    }

    pub fn export(&self) -> Result<String> {
        let data = self.prometheus_exporter.registry().gather();
        TextEncoder::new()
            .encode_to_string(&data)
            .map_err(Error::Prometheus)
    }
}
