use {
    chrono::{DateTime, Duration, Utc},
    ed25519_dalek::PublicKey,
    std::ops::Add,
};

const PUBLIC_KEY_TTL_HOURS: i64 = 6;

#[derive(Clone)]
pub struct RelayClient {
    http_client: reqwest::Client,
    base_url: String,
    public_key: Option<PublicKey>,
    public_key_last_fetched: DateTime<Utc>,
}

impl RelayClient {
    pub fn new(base_url: String) -> RelayClient {
        RelayClient {
            http_client: reqwest::Client::new(),
            base_url,
            public_key: None,
            public_key_last_fetched: DateTime::<Utc>::MIN_UTC,
        }
    }

    /// Fetches the public key with a TTL
    pub async fn public_key(&mut self) -> crate::error::Result<PublicKey> {
        if let Some(public_key) = self.public_key {
            // TTL Not exceeded
            if self
                .public_key_last_fetched
                .add(Duration::hours(PUBLIC_KEY_TTL_HOURS))
                < Utc::now()
            {
                return Ok(public_key);
            }
        }

        let public_key = self.fetch_public_key().await?;
        self.public_key = Some(public_key);
        self.public_key_last_fetched = Utc::now();
        Ok(public_key)
    }

    async fn fetch_public_key(&self) -> crate::error::Result<PublicKey> {
        let response = self
            .http_client
            .get(self.get_url("public-key"))
            .send()
            .await?;
        let body = response.text().await?;
        let key_bytes = hex::decode(body)?;
        let public_key = PublicKey::from_bytes(&key_bytes)?;
        Ok(public_key)
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }
}
