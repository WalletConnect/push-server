use std::collections::HashMap;
use crate::providers::PushProvider;

pub struct NoopProvider {
    // token -> [message, message, message]
    notifications: HashMap<String, Vec<String>>
}

pub fn new() -> NoopProvider {
    NoopProvider {
        notifications: HashMap::new()
    }
}

impl PushProvider for NoopProvider {
    fn send_notification(&mut self, token: String, message: String) -> crate::error::Result<()> {
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
        if !self.notifications.contains_key(&token) {
            self.notifications.insert(token, vec![]);
        }
    }
}

// Debug methods for testing
impl NoopProvider {
    /// Get notifications for a specific token
    fn get_notifications(&mut self, token: String) -> &Vec<String> {
        self.bootstrap(token.clone());

        self.notifications.get(&token).unwrap()
    }
}