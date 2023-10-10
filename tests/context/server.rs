use {
    echo_server::config::Config,
    std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener},
    tokio::{
        runtime::Handle,
        sync::broadcast,
        time::{sleep, Duration},
    },
};

pub struct EchoServer {
    pub public_addr: SocketAddr,
    shutdown_signal: tokio::sync::broadcast::Sender<()>,
    is_shutdown: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl EchoServer {
    pub async fn start(config_from_env: &Config) -> Self {
        let public_port = get_random_port();
        let config = Config {
            port: public_port,
            public_url: format!("http://127.0.0.1:{public_port}"),
            log_level: "info,echo-server=info".into(),
            log_level_otel: "info,echo-server=trace".into(),
            disable_header: true,
            relay_public_key: config_from_env.relay_public_key.clone(),
            validate_signatures: false,
            database_url: config_from_env.database_url.clone(),
            tenant_database_url: config_from_env.tenant_database_url.clone(),
            #[cfg(feature = "multitenant")]
            jwt_secret: "n/a".to_string(),
            otel_exporter_otlp_endpoint: None,
            telemetry_prometheus_port: Some(get_random_port()),
            #[cfg(not(feature = "multitenant"))]
            apns_type: None,
            #[cfg(not(feature = "multitenant"))]
            apns_certificate: None,
            #[cfg(not(feature = "multitenant"))]
            apns_certificate_password: None,
            #[cfg(not(feature = "multitenant"))]
            apns_pkcs8_pem: None,
            #[cfg(not(feature = "multitenant"))]
            apns_team_id: None,
            #[cfg(not(feature = "multitenant"))]
            apns_key_id: None,
            #[cfg(not(feature = "multitenant"))]
            apns_topic: None,
            #[cfg(not(feature = "multitenant"))]
            fcm_api_key: None,
            #[cfg(any(feature = "analytics", feature = "geoblock"))]
            s3_endpoint: None,
            #[cfg(any(feature = "analytics", feature = "geoblock"))]
            geoip_db_bucket: None,
            #[cfg(any(feature = "analytics", feature = "geoblock"))]
            geoip_db_key: None,
            #[cfg(feature = "analytics")]
            analytics_export_bucket: "example-bucket".to_string(),
            is_test: true,
            cors_allowed_origins: vec!["*".to_string()],
            #[cfg(feature = "cloud")]
            cloud_api_url: "https://example.com".to_string(),
            #[cfg(feature = "cloud")]
            cloud_api_key: "n/a".to_string(),
            #[cfg(feature = "geoblock")]
            blocked_countries: vec![],
        };
        let (public_addr, signal, is_shutdown) = start_server(config).await;

        Self {
            public_addr,
            shutdown_signal: signal,
            is_shutdown,
        }
    }

    pub async fn shutdown(&mut self) {
        if self.is_shutdown {
            return;
        }
        self.is_shutdown = true;
        let _ = self.shutdown_signal.send(());
        wait_for_server_to_shutdown(self.public_addr.port())
            .await
            .unwrap();
    }
}

async fn start_server(
    config: Config,
) -> (
    std::net::SocketAddr,
    tokio::sync::broadcast::Sender<()>,
    bool,
) {
    let rt = Handle::current();
    let port = config.port.clone();
    let public_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

    let (signal, shutdown) = broadcast::channel(1);

    std::thread::spawn(move || {
        rt.block_on(async move { echo_server::bootstap(shutdown, config).await })
            .unwrap();
    });

    if let Err(e) = wait_for_server_to_start(port).await {
        panic!("Failed to start server with error: {e:?}")
    }

    (public_addr, signal, false)
}

// Finds a free port.
fn get_random_port() -> u16 {
    use std::sync::atomic::{AtomicU16, Ordering};

    static NEXT_PORT: AtomicU16 = AtomicU16::new(9000);

    loop {
        let port = NEXT_PORT.fetch_add(1, Ordering::SeqCst);

        if is_port_available(port) {
            return port;
        }
    }
}

fn is_port_available(port: u16) -> bool {
    TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)).is_ok()
}

async fn wait_for_server_to_shutdown(port: u16) -> crate::ErrorResult<()> {
    let poll_fut = async {
        while !is_port_available(port) {
            sleep(Duration::from_millis(10)).await;
        }
    };

    Ok(tokio::time::timeout(Duration::from_secs(3), poll_fut).await?)
}

async fn wait_for_server_to_start(port: u16) -> crate::ErrorResult<()> {
    let poll_fut = async {
        while is_port_available(port) {
            sleep(Duration::from_millis(10)).await;
        }
    };

    Ok(tokio::time::timeout(Duration::from_secs(5), poll_fut).await?)
}
