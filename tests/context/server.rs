use {
    crate::context::{DATABASE_URL, TENANT_DATABASE_URL},
    echo_server::config::Config,
    std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener},
    tokio::{
        runtime::Handle,
        sync::broadcast,
        time::{sleep, Duration},
    },
};

pub struct SingleTenantEchoServer {
    pub public_addr: SocketAddr,
    shutdown_signal: tokio::sync::broadcast::Sender<()>,
    is_shutdown: bool,
}

pub struct MultiTenantEchoServer {
    pub public_addr: SocketAddr,
    shutdown_signal: tokio::sync::broadcast::Sender<()>,
    is_shutdown: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl SingleTenantEchoServer {
    pub async fn start() -> Self {
        let public_port = get_random_port();
        let config: Config = Config {
            port: public_port,
            public_url: format!("http://127.0.0.1:{public_port}"),
            log_level: "info,echo-server=info".into(),
            log_level_otel: "info,echo-server=trace".into(),
            disable_header: true,
            relay_url: "https://relay.walletconnect.com".into(),
            validate_signatures: false,
            database_url: "postgres://postgres:root@localhost:5432/postgres".into(),
            tenant_database_url: None,
            default_tenant_id: "https://relay.walletconnect.com".into(),
            otel_exporter_otlp_endpoint: None,
            telemetry_prometheus_port: Some(get_random_port()),
            apns_certificate: None,
            apns_certificate_password: None,
            apns_pkcs8_pem: None,
            apns_key_id: None,
            apns_topic: None,
            fcm_api_key: None,
            analytics_enabled: false,
            analytics_s3_endpoint: None,
            analytics_export_bucket: None,
            analytics_geoip_db_bucket: None,
            analytics_geoip_db_key: None,
            is_test: true,
            cors_allowed_origins: vec!["*".to_string()],
            apns_type: None,
            apns_team_id: None,
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

impl MultiTenantEchoServer {
    pub async fn start() -> Self {
        let public_port = get_random_port();
        let config: Config = Config {
            port: public_port,
            public_url: format!("http://127.0.0.1:{public_port}"),
            log_level: "info,echo-server=info".into(),
            log_level_otel: "info,echo-server=trace".into(),
            disable_header: true,
            relay_url: "https://relay.walletconnect.com".into(),
            validate_signatures: false,
            database_url: DATABASE_URL.into(),
            tenant_database_url: Some(TENANT_DATABASE_URL.into()),
            default_tenant_id: "9bfe94c9cbf74aaa0597094ef561f703".into(),
            otel_exporter_otlp_endpoint: None,
            telemetry_prometheus_port: Some(get_random_port()),
            apns_certificate: None,
            apns_certificate_password: None,
            apns_pkcs8_pem: None,
            apns_key_id: None,
            apns_topic: None,
            fcm_api_key: None,
            analytics_enabled: false,
            analytics_s3_endpoint: None,
            analytics_export_bucket: None,
            analytics_geoip_db_bucket: None,
            analytics_geoip_db_key: None,
            is_test: true,
            cors_allowed_origins: vec!["*".to_string()],
            apns_type: None,
            apns_team_id: None,
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
