#[cfg(test)]
mod fcm_test {
    use std::collections::HashMap;
    use std::sync::Arc;
    use crate::Client;
    use crate::env::get_config;
    use crate::providers::fcm::FcmProvider;
    use crate::providers::{get_provider, Provider};
    use crate::state::new_state;

    #[test]
    fn fetch_provider() {
        let config = get_config().expect("Failed to get config");
        let store: HashMap<String, Client> = HashMap::new();
        let state = new_state(config, store).expect("Failed to create state");
        let state_arc = Arc::new(state);

        let provider = get_provider("fcm".to_string(), &state_arc).expect("Failed to fetch fcm provider");

        match provider {
            Provider::Fcm(p) => {
                assert_eq!(p, FcmProvider::new())
            }
            _ => panic!("`get_provider` didn't return a fcm provider"),
        }
    }
}
