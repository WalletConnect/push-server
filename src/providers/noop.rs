use {
    super::PushMessage, crate::providers::PushProvider, async_trait::async_trait, reqwest::Url,
    std::collections::HashMap, tracing::instrument,
};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct NoopProvider {
    notifications: HashMap<String, Vec<PushMessage>>,
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
        &mut self,
        token: String,
        body: PushMessage,
    ) -> crate::error::Result<()> {
        self.bootstrap(token.clone());

        let notifications = self.notifications.get_mut(&token).unwrap();
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
    fn bootstrap(&mut self, token: String) {
        self.notifications.entry(token).or_default();
    }
}
