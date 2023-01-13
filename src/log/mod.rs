//! This library serves as a thin, opinionated wrapper over the underlying
//! logger apparatus. By default, this crate only exports the various macros,
//! traits and types used in library logging.
//!
//! However, the top level binary may enable the "logger" feature to gain access
//! to the machinery for initializing the global logger.
//!
//! There also some other utility functions that may be accessed by their
//! feature gate. See the [features] section of Cargo.toml for more.
pub use tracing::{debug, error, info, trace, warn};
use {
    opentelemetry::sdk::trace,
    opentelemetry_otlp::WithExportConfig,
    tracing_appender::non_blocking::WorkerGuard,
    tracing_subscriber::{prelude::*, EnvFilter},
};

pub mod prelude {
    //! Reexport of the most common macros and traits used for logging.
    //!
    //! Typically you may simply add `use log::prelude::*` and get access to all
    //! of the usual macros (info!, error!, debug!, etc).

    pub use tracing::{debug, error, info, trace, warn};
}

/// The default log level for the stderr logger, which is used as a fallback if
/// no other can be found.
const DEFAULT_LOG_LEVEL_STDERR: tracing::Level = tracing::Level::WARN;

/// The default log level for the telemetry logger, which is used as a fallback
/// if no other can be found.
const DEFAULT_LOG_LEVEL_OTEL: tracing::Level = tracing::Level::WARN;

/// The environment variable used to control the stderr logger.
const ENV_LOG_LEVEL_STDERR: &str = "LOG_LEVEL";

/// The environment variable used to control the telemetry logger.
const ENV_LOG_LEVEL_OTEL: &str = "LOG_LEVEL_OTEL";

/// The endpoint for the OpenTelemetry gRPC collector, e.g. "localhost:4317".
const OTEL_EXPORTER_OTLP_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";

pub struct Logger {
    _guard: WorkerGuard,
}

impl Logger {
    pub fn init() -> crate::error::Result<Self> {
        let stderr_filter = EnvFilter::try_from_env(ENV_LOG_LEVEL_STDERR)
            .unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL_STDERR.to_string()));

        let (writer, guard) = tracing_appender::non_blocking(std::io::stderr());

        let logger = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_ansi(atty::is(atty::Stream::Stderr))
            .with_writer(writer)
            .with_filter(stderr_filter)
            .boxed();

        let subscriber = tracing_subscriber::registry().with(logger);

        if std::env::var(OTEL_EXPORTER_OTLP_ENDPOINT).is_ok() {
            let telemetry = {
                let tracer = opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
                    .with_trace_config(
                        trace::config().with_id_generator(trace::XrayIdGenerator::default()),
                    )
                    .install_batch(opentelemetry::runtime::Tokio)?;

                tracing_opentelemetry::layer()
                    .with_tracer(tracer)
                    .with_filter(
                        EnvFilter::try_from_env(ENV_LOG_LEVEL_OTEL)
                            .unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL_OTEL.to_string())),
                    )
                    .boxed()
            };

            subscriber.with(telemetry).init();
        } else {
            subscriber.init();
        };

        Ok(Self { _guard: guard })
    }

    pub fn stop(self) {
        // Consume self to trigger drop.
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        opentelemetry::global::shutdown_tracer_provider();
    }
}
