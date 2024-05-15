use {
    super::PushMessage,
    crate::providers::PushProvider,
    async_trait::async_trait,
    reqwest::Url,
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLock,
    tracing::instrument,
};

#[derive(Debug, Default, Clone)]
pub struct NoopProvider {
    notifications: Arc<RwLock<HashMap<String, Vec<PushMessage>>>>,
}

impl NoopProvider {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl PushProvider for NoopProvider {
    #[instrument(name = "send_noop_notification")]
    async fn send_notification(
        &self,
        token: String,
        body: PushMessage,
    ) -> crate::error::Result<()> {
        self.bootstrap(token.clone()).await;

        let mut lock = self.notifications.write().await;
        let notifications = lock.get_mut(&token).unwrap();
        notifications.append(&mut vec![body]);

        if let Ok(url) = token.parse::<Url>() {
            assert!(reqwest::get(url).await?.status().is_success());
        }

        Ok(())
    }
}

// Utils
impl NoopProvider {
    /// Insert empty notifications for a new token
    async fn bootstrap(&self, token: String) {
        self.notifications.write().await.entry(token).or_default();
    }
}
