use {
    crate::{
        analytics::{client_info::ClientInfo, message_info::MessageInfo},
        config::Config,
        log::prelude::*,
    },
    aws_sdk_s3::Client as S3Client,
    std::{net::IpAddr, sync::Arc, time::Duration},
    wc::{
        analytics::{
            self, AnalyticsExt, ArcCollector, AwsConfig, AwsExporter, BatchCollector,
            BatchObserver, CollectionObserver, Collector, CollectorConfig, ExportObserver,
            ParquetBatchFactory,
        },
        geoip::{self, MaxMindResolver, Resolver},
        metrics::otel,
    },
};

pub mod client_info;
pub mod message_info;

const ANALYTICS_EXPORT_TIMEOUT: Duration = Duration::from_secs(30);
const DATA_QUEUE_CAPACITY: usize = 8192;

#[derive(Clone, Copy)]
enum DataKind {
    Messages,
    Clients,
}

impl DataKind {
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            Self::Messages => "messages",
            Self::Clients => "clients",
        }
    }

    #[inline]
    fn as_kv(&self) -> otel::KeyValue {
        otel::KeyValue::new("data_kind", self.as_str())
    }
}

fn success_kv(success: bool) -> otel::KeyValue {
    otel::KeyValue::new("success", success)
}

#[derive(Clone, Copy)]
struct Observer(DataKind);

impl<T, E> BatchObserver<T, E> for Observer
where
    E: std::error::Error,
{
    fn observe_batch_serialization(&self, elapsed: Duration, res: &Result<Vec<u8>, E>) {
        let size = res.as_deref().map(|data| data.len()).unwrap_or(0);
        let elapsed = elapsed.as_millis() as u64;

        wc::metrics::counter!(
            "analytics_batches_finished",
            1,
            &[self.0.as_kv(), success_kv(res.is_ok())]
        );

        if let Err(err) = res {
            tracing::warn!(
                ?err,
                data_kind = self.0.as_str(),
                "failed to serialize analytics batch"
            );
        } else {
            tracing::info!(
                size,
                elapsed,
                data_kind = self.0.as_str(),
                "analytics data batch serialized"
            );
        }
    }
}

impl<T, E> CollectionObserver<T, E> for Observer
where
    E: std::error::Error,
{
    fn observe_collection(&self, res: &Result<(), E>) {
        wc::metrics::counter!(
            "analytics_records_collected",
            1,
            &[self.0.as_kv(), success_kv(res.is_ok())]
        );

        if let Err(err) = res {
            tracing::warn!(
                ?err,
                data_kind = self.0.as_str(),
                "failed to collect analytics data"
            );
        }
    }
}

impl<E> ExportObserver<E> for Observer
where
    E: std::error::Error,
{
    fn observe_export(&self, elapsed: Duration, res: &Result<(), E>) {
        wc::metrics::counter!(
            "analytics_batches_exported",
            1,
            &[self.0.as_kv(), success_kv(res.is_ok())]
        );

        let elapsed = elapsed.as_millis() as u64;

        if let Err(err) = res {
            tracing::warn!(
                ?err,
                elapsed,
                data_kind = self.0.as_str(),
                "analytics export failed"
            );
        } else {
            tracing::info!(
                elapsed,
                data_kind = self.0.as_str(),
                "analytics export failed"
            );
        }
    }
}

#[derive(Clone)]
pub struct PushAnalytics {
    pub messages: ArcCollector<MessageInfo>,
    pub clients: ArcCollector<ClientInfo>,
    pub geoip_resolver: Option<Arc<MaxMindResolver>>,
}

impl PushAnalytics {
    pub fn with_noop_export() -> Self {
        info!("initializing analytics with noop export");

        Self {
            messages: analytics::noop_collector().boxed_shared(),
            clients: analytics::noop_collector().boxed_shared(),
            geoip_resolver: None,
        }
    }

    pub fn with_aws_export(
        s3_client: S3Client,
        export_bucket: &str,
        node_addr: IpAddr,
        geoip_resolver: Option<Arc<MaxMindResolver>>,
    ) -> Self {
        let messages = {
            let data_kind = DataKind::Messages;
            let observer = Observer(data_kind);
            BatchCollector::new(
                CollectorConfig {
                    data_queue_capacity: DATA_QUEUE_CAPACITY,
                    ..Default::default()
                },
                ParquetBatchFactory::new(Default::default()).with_observer(observer),
                AwsExporter::new(AwsConfig {
                    export_prefix: "echo/messages".to_string(),
                    export_name: "push_messages".to_string(),
                    node_addr,
                    file_extension: "parquet".to_owned(),
                    bucket_name: export_bucket.to_owned(),
                    s3_client: s3_client.clone(),
                    upload_timeout: ANALYTICS_EXPORT_TIMEOUT,
                })
                .with_observer(observer),
            )
            .with_observer(observer)
            .boxed_shared()
        };

        let clients = {
            let data_kind = DataKind::Clients;
            let observer = Observer(data_kind);
            BatchCollector::new(
                CollectorConfig {
                    data_queue_capacity: DATA_QUEUE_CAPACITY,
                    ..Default::default()
                },
                ParquetBatchFactory::new(Default::default()).with_observer(observer),
                AwsExporter::new(AwsConfig {
                    export_prefix: "echo/clients".to_string(),
                    export_name: "push_clients".to_string(),
                    node_addr,
                    file_extension: "parquet".to_owned(),
                    bucket_name: export_bucket.to_owned(),
                    s3_client: s3_client.clone(),
                    upload_timeout: ANALYTICS_EXPORT_TIMEOUT,
                })
                .with_observer(observer),
            )
            .with_observer(observer)
            .boxed_shared()
        };

        Self {
            messages,
            clients,
            geoip_resolver,
        }
    }

    pub fn message(&self, data: MessageInfo) {
        if let Err(err) = self.messages.collect(data) {
            tracing::warn!(
                ?err,
                data_kind = DataKind::Messages.as_str(),
                "failed to collect analytics"
            );
        }
    }

    pub fn client(&self, data: ClientInfo) {
        if let Err(err) = self.clients.collect(data) {
            tracing::warn!(
                ?err,
                data_kind = DataKind::Clients.as_str(),
                "failed to collect analytics"
            );
        }
    }

    pub fn lookup_geo_data(&self, addr: IpAddr) -> Option<geoip::Data> {
        self.geoip_resolver
            .as_ref()?
            .lookup_geo_data(addr)
            .map_err(|err| {
                error!(?err, "failed to lookup geoip data");
                err
            })
            .ok()
    }
}

pub async fn initialize(
    config: &Config,
    s3_client: S3Client,
    echo_ip: IpAddr,
    geoip_resolver: Option<Arc<MaxMindResolver>>,
) -> PushAnalytics {
    PushAnalytics::with_aws_export(
        s3_client,
        &config.analytics_export_bucket,
        echo_ip,
        geoip_resolver,
    )
}
