#[cfg(test)]
mod noop_test {
    use crate::providers::noop;
    use crate::providers::{get_provider, Provider, PushProvider};

    const PUSH_TOKEN: &str = "0000-0000-0000-0000";
    const EXAMPLE_MESSAGE: &str = "You have a signing request.";

    #[test]
    fn fetch_provider() {
        let provider = get_provider("noop".to_string());
        if provider.is_err() {
            panic!("Failed to fetch noop provider")
        }

        match provider.unwrap() {
            Provider::Noop(p) => {
                assert_eq!(p, noop::new())
            }
            _ => panic!("`get_provider` didn't return a noop provider"),
        }
    }

    #[test]
    fn send_notification() {
        let mut provider = noop::new();

        provider
            .send_notification(PUSH_TOKEN.to_string(), EXAMPLE_MESSAGE.to_string())
            .expect("Failed to send notification");

        let notifications = provider.get_notifications(PUSH_TOKEN.to_string());

        assert_eq!(notifications.len(), 1)
    }
}
