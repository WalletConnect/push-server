use {
    crate::{
        analytics::{client_info::ClientInfo, message_info::MessageInfo},
        config::Config,
        error::{Error, Result},
        log::prelude::*,
    },
    aws_config::meta::region::RegionProviderChain,
    aws_sdk_s3::{config::Region, Client as S3Client},
    gorgon::{
        collectors::{batch::BatchOpts, noop::NoopCollector},
        exporters::aws::{AwsExporter, AwsOpts},
        geoip::{AnalyticsGeoData, GeoIpReader},
        writers::parquet::ParquetWriter,
        Analytics,
    },
    std::{net::IpAddr, sync::Arc},
};

pub mod client_info;
pub mod message_info;

#[derive(Clone)]
pub struct PushAnalytics {
    pub messages: Analytics<MessageInfo>,
    pub clients: Analytics<ClientInfo>,
    pub geoip: GeoIpReader,
}

impl PushAnalytics {
    pub fn with_noop_export() -> Self {
        info!("initializing analytics with noop export");

        Self {
            messages: Analytics::new(NoopCollector),
            clients: Analytics::new(NoopCollector),
            geoip: GeoIpReader::empty(),
        }
    }

    pub fn with_aws_export(
        s3_client: S3Client,
        export_bucket: &str,
        node_ip: IpAddr,
        geoip: GeoIpReader,
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
            geoip,
        })
    }

    pub fn message(&self, data: MessageInfo) {
        self.messages.collect(data);
    }

    pub fn client(&self, data: ClientInfo) {
        self.clients.collect(data);
    }

    pub fn lookup_geo_data(&self, addr: IpAddr) -> Option<AnalyticsGeoData> {
        self.geoip.lookup_geo_data_with_city(addr)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AnalyticsConfig {
    pub s3_endpoint: Option<String>,
    pub export_bucket: Option<String>,
    pub geoip_db_bucket: Option<String>,
    pub geoip_db_key: Option<String>,
}

pub async fn initialize(config: &Config, echo_ip: IpAddr) -> Result<PushAnalytics> {
    let analytics = AnalyticsConfig {
        s3_endpoint: config.analytics_s3_endpoint.clone(),
        export_bucket: Some(config.analytics_export_bucket.clone()),
        geoip_db_bucket: config.analytics_geoip_db_bucket.clone(),
        geoip_db_key: config.analytics_geoip_db_key.clone(),
    };

    if let Some(export_bucket) = analytics.export_bucket.as_deref() {
        let region_provider = RegionProviderChain::first_try(Region::new("eu-central-1"));
        let shared_config = aws_config::from_env().region(region_provider).load().await;

        let aws_config = if let Some(s3_endpoint) = &analytics.s3_endpoint {
            info!(%s3_endpoint, "initializing analytics with custom s3 endpoint");

            aws_sdk_s3::config::Builder::from(&shared_config)
                .endpoint_url(s3_endpoint)
                .build()
        } else {
            aws_sdk_s3::config::Builder::from(&shared_config).build()
        };

        let s3_client = S3Client::from_conf(aws_config);
        let geoip_params = (&analytics.geoip_db_bucket, &analytics.geoip_db_key);

        let geoip = if let (Some(bucket), Some(key)) = geoip_params {
            info!(%bucket, %key, "initializing geoip database from aws s3");

            GeoIpReader::from_aws_s3(&s3_client, bucket, key)
                .await
                .map_err(|_| Error::GeoIpS3Failed)?
        } else {
            info!("analytics geoip lookup is disabled");

            GeoIpReader::empty()
        };

        PushAnalytics::with_aws_export(s3_client, export_bucket, echo_ip, geoip)
    } else {
        Ok(PushAnalytics::with_noop_export())
    }
}
