use crate::store::MockStore;
use echo_server::providers::noop::NoopProvider;
use echo_server::providers::{get_provider, Provider};
use echo_server::state::new_state;
use echo_server::{env::get_config, providers::ProviderKind};
use std::env;
use std::sync::Arc;

// const PUSH_TOKEN: &str = "0000-0000-0000-0000";
// const EXAMPLE_MESSAGE: &str = "You have a signing request.";

#[test]
fn fetch_provider() {
    // Reset the env vars
    crate::env::reset_env();

    env::set_var("DATABASE_URL", "");
    let config = get_config().expect("Failed to get config");
    let store = MockStore::new();
    let state = new_state(config, store).expect("Failed to create state");
    let state_arc = Arc::new(state);

    let provider =
        get_provider(ProviderKind::Noop, &state_arc).expect("Failed to fetch noop provider");

    match provider {
        Provider::Noop(p) => {
            assert_eq!(p, NoopProvider::new())
        }
        _ => panic!("`get_provider` didn't return a noop provider"),
    }
}
