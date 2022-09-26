// use echo_server::env::get_config;
// use echo_server::providers::{get_provider, Provider};
// use echo_server::state::new_state;
// use std::collections::HashMap;
// use std::sync::Arc;

// TODO Fix APNS Env/GH Actions Test Environment
// #[test]
// /// For this test too pass you must set APNS env vars, either for a cert or token client.
// fn fetch_provider() {
//     let config = get_config().expect("Failed to get config");
//     let store: HashMap<String, Client> = HashMap::new();
//     let state = new_state(config, store).expect("Failed to create state");
//     let state_arc = Arc::new(state);
//     let provider = get_provider("apns", &state_arc).expect("Failed to fetch apns provider");
//
//     match provider {
//         Provider::Apns(_) => {}
//         _ => panic!("`get_provider` didn't return an apns provider"),
//     }
// }
