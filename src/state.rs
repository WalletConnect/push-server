use crate::{BuildInfo, Config, error};

pub struct Metrics {
    pub registered_webhooks: opentelemetry::metrics::Counter<u64>,
    pub received_notifications: opentelemetry::metrics::Counter<u64>
}

pub struct State {
    pub config: Config,
    pub build_info: BuildInfo,
    redis: redis::Client,
    pub tracer: Option<opentelemetry::sdk::trace::Tracer>,
    pub metrics: Option<Metrics>
}

build_info::build_info!(fn build_info);

pub fn new_state(config: Config, redis_client: redis::Client) -> State {
    let build_info: &BuildInfo = build_info();

    State {
        config,
        build_info: build_info.clone(),
        redis: redis_client,
        tracer: None,
        metrics: None
    }
}

impl State {
    pub fn get_redis_connection(&self) -> error::Result<redis::Connection> {
        Ok(self.redis.get_connection()?)
    }

    pub fn set_telemetry(&mut self, tracer: opentelemetry::sdk::trace::Tracer, metrics: Metrics) {
        self.tracer = Some(tracer);
        self.metrics = Some(metrics);
    }
}