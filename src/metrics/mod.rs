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

    pub received_notifications: Counter<u64>,
    pub sent_fcm_notifications: Counter<u64>,
    pub sent_fcm_v1_notifications: Counter<u64>,
    pub sent_apns_notifications: Counter<u64>,

    pub registered_clients: UpDownCounter<i64>,
    pub registered_tenants: UpDownCounter<i64>,

    pub tenant_apns_updates: Counter<u64>,
    pub tenant_fcm_updates: Counter<u64>,
    pub tenant_fcm_v1_updates: Counter<u64>,

    pub tenant_suspensions: Counter<u64>,
    pub client_suspensions: Counter<u64>,
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

        let tenants_counter = meter
            .i64_up_down_counter("registered_tenants")
            .with_description("The number of currently registered tenants")
            .init();

        let received_notification_counter = meter
            .u64_counter("received_notifications")
            .with_description("The number of notification received")
            .init();

        let sent_fcm_notification_counter = meter
            .u64_counter("sent_fcm_notifications")
            .with_description("The number of notifications sent to FCM")
            .init();

        let sent_fcm_v1_notification_counter = meter
            .u64_counter("sent_fcm_v1_notifications")
            .with_description("The number of notifications sent to FCM")
            .init();

        let sent_apns_notification_counter = meter
            .u64_counter("sent_apns_notifications")
            .with_description("The number of notifications sent to APNS")
            .init();

        let tenant_apns_updates_counter = meter
            .u64_counter("tenant_apns_updates")
            .with_description("The number of times tenants have updated their APNS")
            .init();

        let tenant_fcm_updates_counter = meter
            .u64_counter("tenant_fcm_updates")
            .with_description("The number of times tenants have updated their FCM")
            .init();

        let tenant_fcm_v1_updates_counter = meter
            .u64_counter("tenant_fcm_v1_updates")
            .with_description("The number of times tenants have updated their FCM")
            .init();

        let tenant_suspensions_counter = meter
            .u64_counter("tenant_suspensions")
            .with_description("The number of tenants that have been suspended")
            .init();

        let client_suspensions_counter = meter
            .u64_counter("client_suspensions")
            .with_description("The number of clients that have been suspended")
            .init();

        Ok(Metrics {
            prometheus_exporter,
            registered_clients: clients_counter,
            received_notifications: received_notification_counter,
            sent_fcm_notifications: sent_fcm_notification_counter,
            sent_fcm_v1_notifications: sent_fcm_v1_notification_counter,
            sent_apns_notifications: sent_apns_notification_counter,
            registered_tenants: tenants_counter,
            tenant_apns_updates: tenant_apns_updates_counter,
            tenant_fcm_updates: tenant_fcm_updates_counter,
            tenant_fcm_v1_updates: tenant_fcm_v1_updates_counter,
            tenant_suspensions: tenant_suspensions_counter,
            client_suspensions: client_suspensions_counter,
        })
    }

    pub fn export(&self) -> Result<String> {
        let data = self.prometheus_exporter.registry().gather();
        TextEncoder::new()
            .encode_to_string(&data)
            .map_err(Error::Prometheus)
    }
}
