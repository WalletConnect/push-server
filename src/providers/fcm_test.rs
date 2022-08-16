#[cfg(test)]
mod fcm_test {
    use crate::env::get_config;
    use crate::providers::fcm::FcmProvider;
    use crate::providers::{get_provider, Provider};
    use crate::state::new_state;
    use crate::Client;
    use std::collections::HashMap;
    use std::env;
    use std::sync::Arc;

    const FCM_API_KEY: &str = "API0-KEY0-0000-000";

    #[test]
    fn fetch_provider() {
        env::set_var("FCM_API_KEY", FCM_API_KEY);

        let config = get_config().expect("Failed to get config");
        let store: HashMap<String, Client> = HashMap::new();
        let state = new_state(config, store).expect("Failed to create state");
        let state_arc = Arc::new(state);

        let provider =
            get_provider("fcm".to_string(), &state_arc).expect("Failed to fetch fcm provider");

        match provider {
            Provider::Fcm(p) => {
                assert_eq!(p, FcmProvider::new(FCM_API_KEY.to_string()))
            }
            _ => panic!("`get_provider` didn't return a fcm provider"),
        }
    }
}
