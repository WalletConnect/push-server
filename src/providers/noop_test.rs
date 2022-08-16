#[cfg(test)]
mod noop_test {
    use std::collections::HashMap;
    use std::sync::Arc;
    use crate::Client;
    use crate::env::get_config;
    use crate::providers::noop::NoopProvider;
    use crate::providers::{get_provider, Provider, PushProvider};
    use crate::state::new_state;

    const PUSH_TOKEN: &str = "0000-0000-0000-0000";
    const EXAMPLE_MESSAGE: &str = "You have a signing request.";

    #[test]
    fn fetch_provider() {
        let config = get_config().expect("Failed to get config");
        let store: HashMap<String, Client> = HashMap::new();
        let state = new_state(config, store).expect("Failed to create state");
        let state_arc = Arc::new(state);

        let provider = get_provider("noop".to_string(), &state_arc).expect("Failed to fetch noop provider");

        match provider {
            Provider::Noop(p) => {
                assert_eq!(p, NoopProvider::new())
            }
            _ => panic!("`get_provider` didn't return a noop provider"),
        }
    }

    #[test]
    fn send_notification() {
        let mut provider = NoopProvider::new();

        provider
            .send_notification(PUSH_TOKEN.to_string(), EXAMPLE_MESSAGE.to_string())
            .expect("Failed to send notification");

        let notifications = provider.get_notifications(PUSH_TOKEN.to_string());

        assert_eq!(notifications.len(), 1)
    }
}
