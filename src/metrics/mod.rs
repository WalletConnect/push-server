use {
    crate::error::{Error, Result},
    opentelemetry::{
        metrics::{Counter, UpDownCounter},
        sdk::{
            self,
            export::metrics::aggregation,
            metrics::{processors, selectors},
            Resource,
        },
    },
    opentelemetry_prometheus::PrometheusExporter,
    prometheus_core::TextEncoder,
};

#[derive(Clone)]
pub struct Metrics {
    pub prometheus_exporter: PrometheusExporter,
    pub registered_clients: UpDownCounter<i64>,
    pub received_notifications: Counter<u64>,
    pub sent_notifications: Counter<u64>,
}

impl Metrics {
    pub fn new(resource: Resource) -> Result<Self> {
        let controller = sdk::metrics::controllers::basic(
            processors::factory(
                selectors::simple::histogram(vec![]),
                aggregation::cumulative_temporality_selector(),
            )
            .with_memory(true),
        )
        .with_resource(resource)
        .build();

        let prometheus_exporter = opentelemetry_prometheus::exporter(controller).init();

        let meter = prometheus_exporter.meter_provider().unwrap();

        opentelemetry::global::set_meter_provider(meter);

        let meter = opentelemetry::global::meter("echo-server");

        let clients_counter = meter
            .i64_up_down_counter("registered_clients")
            .with_description("The number of currently registered clients")
            .init();

        let received_notification_counter = meter
            .u64_counter("received_notifications")
            .with_description("The number of notification received")
            .init();

        let sent_notification_counter = meter
            .u64_counter("received_notifications")
            .with_description("The number of notification received")
            .init();

        Ok(Metrics {
            prometheus_exporter,
            registered_clients: clients_counter,
            received_notifications: received_notification_counter,
            sent_notifications: sent_notification_counter,
        })
    }

    pub fn export(&self) -> Result<String> {
        let data = self.prometheus_exporter.registry().gather();
        TextEncoder::new()
            .encode_to_string(&data)
            .map_err(Error::Prometheus)
    }
}
