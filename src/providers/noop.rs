use crate::providers::PushProvider;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NoopProvider {
    // token -> [message, message, message]
    notifications: HashMap<String, Vec<String>>,
}

impl NoopProvider {
    pub fn new() -> Self {
        NoopProvider {
            notifications: HashMap::new(),
        }
    }
}

#[async_trait]
impl PushProvider for NoopProvider {
    async fn send_notification(
        &mut self,
        token: String,
        message: String,
    ) -> crate::error::Result<()> {
        self.bootstrap(token.clone());

        let notifications = self.notifications.get_mut(&token).unwrap();
        notifications.append(&mut vec![message]);

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

// Debug methods for testing
#[cfg(test)]
impl NoopProvider {
    /// Get notifications for a specific token
    pub fn get_notifications(&mut self, token: String) -> &Vec<String> {
        self.bootstrap(token.clone());

        self.notifications.get(&token).unwrap()
    }
}
