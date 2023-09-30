use {
    crate::{
        analytics::{client_info::ClientInfo, message_info::MessageInfo},
        config::Config,
        error::Result,
        log::prelude::*,
    },
    aws_sdk_s3::Client as S3Client,
    std::{net::IpAddr, sync::Arc},
    wc::{
        analytics::{
            collectors::{batch::BatchOpts, noop::NoopCollector},
            exporters::aws::{AwsExporter, AwsOpts},
            writers::parquet::ParquetWriter,
            Analytics,
        },
        geoip::{self, MaxMindResolver, Resolver},
    },
};

pub mod client_info;
pub mod message_info;

#[derive(Clone)]
pub struct PushAnalytics {
    pub messages: Analytics<MessageInfo>,
    pub clients: Analytics<ClientInfo>,
    pub geoip_resolver: Option<Arc<MaxMindResolver>>,
}

impl PushAnalytics {
    pub fn with_noop_export() -> Self {
        info!("initializing analytics with noop export");

        Self {
            messages: Analytics::new(NoopCollector),
            clients: Analytics::new(NoopCollector),
            geoip_resolver: None,
        }
    }

    pub fn with_aws_export(
        s3_client: S3Client,
        export_bucket: &str,
        node_ip: IpAddr,
        geoip_resolver: Option<Arc<MaxMindResolver>>,
    ) -> Result<Self> {
        info!(%export_bucket, "initializing analytics with aws export");

        let opts = BatchOpts::default();
        let bucket_name: Arc<str> = export_bucket.into();
        let node_ip: Arc<str> = node_ip.to_string().into();

        let messages = {
            let exporter = AwsExporter::new(AwsOpts {
                export_prefix: "echo/messages",
                export_name: "push_messages",
                file_extension: "parquet",
                bucket_name: bucket_name.clone(),
                s3_client: s3_client.clone(),
                node_ip: node_ip.clone(),
            });

            let collector = ParquetWriter::<MessageInfo>::new(opts.clone(), exporter)?;
            Analytics::new(collector)
        };

        let clients = {
            let exporter = AwsExporter::new(AwsOpts {
                export_prefix: "echo/clients",
                export_name: "push_clients",
                file_extension: "parquet",
                bucket_name,
                s3_client,
                node_ip,
            });

            Analytics::new(ParquetWriter::new(opts, exporter)?)
        };

        Ok(Self {
            messages,
            clients,
            geoip_resolver,
        })
    }

    pub fn message(&self, data: MessageInfo) {
        self.messages.collect(data);
    }

    pub fn client(&self, data: ClientInfo) {
        self.clients.collect(data);
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
) -> Result<PushAnalytics> {
    PushAnalytics::with_aws_export(
        s3_client,
        &config.analytics_export_bucket,
        echo_ip,
        geoip_resolver,
    )
}
