use {
    crate::{config::Config, error::Result, log::prelude::*},
    aws_sdk_s3::{Client as S3Client, Region},
    aws_config::meta::region::RegionProviderChain,
    gorgon::{
        batcher::{AwsExporter, AwsExporterOpts, BatchCollectorOpts},
        geoip::{AnalyticsGeoData, GeoIpReader},
        Analytics,
        NoopCollector,
    },
    std::{net::IpAddr, sync::Arc},
};

pub use {message_info::*};

mod message_info;

#[derive(Clone)]
pub struct PushAnalytics {
    pub messages: Analytics<MessageInfo>,
    pub geoip: GeoIpReader,
}

impl PushAnalytics {
    pub fn with_noop_export() -> Self {
        info!("initializing analytics with noop export");

        Self {
            messages: Analytics::new(NoopCollector),
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

        let opts = BatchCollectorOpts::default();
        let bucket_name: Arc<str> = export_bucket.into();
        let node_ip: Arc<str> = node_ip.to_string().into();

        let messages = {
            let exporter = AwsExporter::new(AwsExporterOpts {
                export_name: "push_messages",
                file_extension: "parquet",
                bucket_name: bucket_name.clone(),
                s3_client: s3_client.clone(),
                node_ip: node_ip.clone(),
            });

            Analytics::new(gorgon::batcher::create_parquet_collector::<MessageInfo, _>(
                opts.clone(),
                exporter,
            )?)
        };

        Ok(Self { messages, geoip })
    }

    pub fn message(&self, data: MessageInfo) {
        self.messages.collect(data);
    }

    pub fn lookup_geo_data(&self, addr: IpAddr) -> Option<AnalyticsGeoData> {
        self.geoip.lookup_geo_data_with_city(addr)
    }
}

pub async fn initialize(config: &Config, echo_ip: IpAddr) -> Result<PushAnalytics> {
    if let Some(export_bucket) = config.analytics_export_bucket.as_deref() {
        let region_provider = RegionProviderChain::first_try(Region::new("eu-central-1"));
        let shared_config = aws_config::from_env().region(region_provider).load().await;

        let aws_config = if let Some(s3_endpoint) = &config.analytics_s3_endpoint {
            info!(%s3_endpoint, "initializing analytics with custom s3 endpoint");

            aws_sdk_s3::config::Builder::from(&shared_config)
                .endpoint_url(s3_endpoint)
                .build()
        } else {
            aws_sdk_s3::config::Builder::from(&shared_config).build()
        };

        let s3_client = S3Client::from_conf(aws_config);
        let geoip_params = (
            &config.analytics_geoip_db_bucket,
            &config.analytics_geoip_db_key,
        );

        let geoip = if let (Some(bucket), Some(key)) = geoip_params {
            info!(%bucket, %key, "initializing geoip database from aws s3");

            GeoIpReader::from_aws_s3(&s3_client, bucket, key).await?
        } else {
            info!("analytics geoip lookup is disabled");

            GeoIpReader::empty()
        };

        PushAnalytics::with_aws_export(s3_client, export_bucket, echo_ip, geoip)
    } else {
        Ok(PushAnalytics::with_noop_export())
    }
}
