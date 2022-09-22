#[cfg(test)]
mod noop_test {
    use crate::env::get_config;
    use crate::providers::noop::NoopProvider;
    use crate::providers::{get_provider, Provider, PushProvider};
    use crate::state::new_state;
    use crate::Client;
    use std::collections::HashMap;
    use std::sync::Arc;

    const PUSH_TOKEN: &str = "0000-0000-0000-0000";
    const EXAMPLE_MESSAGE: &str = "You have a signing request.";

    #[test]
    fn fetch_provider() {
        let config = get_config().expect("Failed to get config");
        let store: HashMap<String, Client> = HashMap::new();
        let state = new_state(config, store).expect("Failed to create state");
        let state_arc = Arc::new(state);

        let provider = get_provider("noop", &state_arc).expect("Failed to fetch noop provider");

        match provider {
            Provider::Noop(p) => {
                assert_eq!(p, NoopProvider::new())
            }
            _ => panic!("`get_provider` didn't return a noop provider"),
        }
    }
}
