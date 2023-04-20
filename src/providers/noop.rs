use {
    crate::{handlers::push_message::MessagePayload, providers::PushProvider},
    async_trait::async_trait,
    std::collections::HashMap,
    tracing::span,
};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct NoopProvider {
    // token -> [MessagePayload{..}, MessagePayload{..}, MessagePayload{..}]
    notifications: HashMap<String, Vec<MessagePayload>>,
}

impl NoopProvider {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl PushProvider for NoopProvider {
    async fn send_notification(
        &mut self,
        token: String,
        payload: MessagePayload,
    ) -> crate::error::Result<()> {
        let s = span!(tracing::Level::DEBUG, "send_noop_notification");
        let _ = s.enter();

        self.bootstrap(token.clone());

        let notifications = self.notifications.get_mut(&token).unwrap();
        notifications.append(&mut vec![payload]);

        Ok(())
    }
}

// Utils
impl NoopProvider {
    /// Insert empty notifications for a new token
    fn bootstrap(&mut self, token: String) {
        self.notifications.entry(token).or_insert_with(Vec::new);
    }
}
