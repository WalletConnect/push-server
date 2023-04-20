use echo_server::{config, relay::RelayClient};

pub fn get_client() -> RelayClient {
    RelayClient::new(config::RELAY_URL.to_string())
}

#[tokio::test]
pub async fn fetch_public_key() {
    let mut client = get_client();

    let res = client.public_key().await;

    assert!(res.is_ok());
}
