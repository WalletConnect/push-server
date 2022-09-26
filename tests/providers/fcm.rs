use crate::store::MockStore;
use echo_server::env::get_config;
use echo_server::providers::{fcm::FcmProvider, ProviderKind};
use echo_server::providers::{get_provider, Provider};
use echo_server::state::new_state;
use serial_test::serial;
use std::env;
use std::sync::Arc;

const FCM_API_KEY: &str = "API0-KEY0-0000-000";

#[test]
#[serial]
fn fetch_provider() {
    // Reset the env vars
    crate::env::reset_env();

    env::set_var("FCM_API_KEY", FCM_API_KEY);

    let config = get_config().expect("Failed to get config");
    let store = MockStore::new();
    let state = new_state(config, store).expect("Failed to create state");
    let state_arc = Arc::new(state);
    let provider =
        get_provider(ProviderKind::Fcm, &state_arc).expect("Failed to fetch fcm provider");

    match provider {
        Provider::Fcm(p) => {
            assert_eq!(p, FcmProvider::new(FCM_API_KEY.to_string()))
        }
        _ => panic!("`get_provider` didn't return a fcm provider"),
    }
}
