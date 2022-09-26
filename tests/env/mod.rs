use echo_server::env::{get_config, Config};
use serial_test::serial;
use std::env;

pub fn reset_env() {
    env::remove_var("PORT");
    env::remove_var("LOG_LEVEL");
    env::remove_var("TELEMETRY_ENABLED");
    env::remove_var("TELEMETRY_GRPC_URL");
    env::remove_var("FCM_API_KEY");
}

#[test]
#[serial]
fn default_config() {
    reset_env();

    let config = get_config().expect("Failed to fetch config");

    assert_eq!(
        config,
        Config {
            port: 3000,
            log_level: "WARN".to_string(),
            database_url: Default::default(),
            telemetry_enabled: None,
            telemetry_grpc_url: None,
            apns_certificate: None,
            apns_token: None,
            fcm_api_key: None
        }
    )
}

#[test]
#[serial]
fn env_config_1() {
    reset_env();

    env::set_var("TELEMETRY_ENABLED", "false");

    let config = get_config().expect("Failed to fetch config");

    assert_eq!(
        config,
        Config {
            port: 3000,
            log_level: "WARN".to_string(),
            database_url: Default::default(),
            telemetry_enabled: Some(false),
            telemetry_grpc_url: None,
            apns_certificate: None,
            apns_token: None,
            fcm_api_key: None
        }
    )
}

#[test]
#[serial]
fn env_config_2() {
    reset_env();

    env::set_var("PORT", "3001");
    env::set_var("LOG_LEVEL", "TRACE");

    let config = get_config().expect("Failed to fetch config");

    assert_eq!(
        config,
        Config {
            port: 3001,
            log_level: "TRACE".to_string(),
            database_url: Default::default(),
            telemetry_enabled: None,
            telemetry_grpc_url: None,
            apns_certificate: None,
            apns_token: None,
            fcm_api_key: None
        }
    )
}

#[test]
#[serial]
fn env_config_3() {
    reset_env();

    env::set_var("PORT", "8080");
    env::set_var("LOG_LEVEL", "ERROR");

    let config = get_config().expect("Failed to fetch config");

    assert_eq!(
        config,
        Config {
            port: 8080,
            log_level: "ERROR".to_string(),
            database_url: Default::default(),
            telemetry_enabled: None,
            telemetry_grpc_url: None,
            apns_certificate: None,
            apns_token: None,
            fcm_api_key: None
        }
    )
}

#[test]
#[serial]
fn env_config_4() {
    reset_env();

    env::set_var("PORT", "3001");
    env::set_var("LOG_LEVEL", "TRACE");
    env::set_var("TELEMETRY_ENABLED", "true");
    env::set_var("TELEMETRY_GRPC_URL", "http://localhost:4317");
    env::set_var("FCM_API_KEY", "API-KEY");

    let config = get_config().expect("Failed to fetch config");

    assert_eq!(
        config,
        Config {
            port: 3001,
            log_level: "TRACE".to_string(),
            database_url: Default::default(),
            telemetry_enabled: Some(true),
            telemetry_grpc_url: Some("http://localhost:4317".to_string()),
            apns_certificate: None,
            apns_token: None,
            fcm_api_key: Some("API-KEY".to_string())
        }
    )
}
