use {
    echo_server::env::Config,
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
    pub async fn start() -> Self {
        let public_port = get_random_port();
        let rt = Handle::current();
        let public_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), public_port);

        let (signal, shutdown) = broadcast::channel(1);

        std::thread::spawn(move || {
            rt.block_on(async move {
                let config: Config = Config {
                    port: public_port,
                    log_level: "INFO".into(),
                    relay_url: "https://relay.walletconnect.com".into(),
                    database_url: "postgres://postgres@localhost:5432/postgres".into(),
                    tenant_database_url: None,
                    default_tenant_id: "https://relay.walletconnect.com".into(),
                    telemetry_enabled: None,
                    telemetry_grpc_url: None,
                    apns_sandbox: true,
                    apns_certificate: None,
                    apns_certificate_password: None,
                    apns_topic: None,
                    fcm_api_key: None,
                    is_test: true,
                };

                echo_server::bootstap(shutdown, config).await
            })
            .unwrap();
        });

        if let Err(e) = wait_for_server_to_start(public_port).await {
            panic!("Failed to start server with error: {:?}", e)
        }

        Self {
            public_addr,
            shutdown_signal: signal,
            is_shutdown: false,
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
