use {
    crate::{context::EchoServerContext, functional::multitenant::generate_random_tenant_id},
    echo_server::{
        handlers::{create_tenant::TenantRegisterBody, get_tenant::GetTenantResponse},
        providers::PROVIDER_APNS,
    },
    std::env,
    test_context::test_context,
};

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_apns_valid_token(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register new tenant
    let client = reqwest::Client::new();
    let create_tenant_result = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Failed to create a new tenant");
    assert_eq!(create_tenant_result.status(), reqwest::StatusCode::OK);

    // Send valid APNS p8 Key
    let form = reqwest::multipart::Form::new()
        .text("apns_type", "token")
        .text("apns_topic", "app.test")
        .text("apns_key_id", env::var("ECHO_TEST_APNS_P8_KEY_ID").unwrap())
        .text(
            "apns_team_id",
            env::var("ECHO_TEST_APNS_P8_TEAM_ID").unwrap(),
        )
        .part(
            "apns_pkcs8_pem",
            reqwest::multipart::Part::text(env::var("ECHO_TEST_APNS_P8_PEM").unwrap())
                .file_name("apns.p8")
                .mime_str("text/plain")
                .expect("Error on passing multipart stream to the form request"),
        );
    let apns_update_result = client
        .post(format!(
            "http://{}/tenants/{}/apns",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Failed to call update tenant endpoint");
    assert_eq!(apns_update_result.status(), reqwest::StatusCode::OK);
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_enabled_providers(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register new tenant
    let client = reqwest::Client::new();
    let create_tenant_result = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Failed to create a new tenant");
    assert_eq!(create_tenant_result.status(), reqwest::StatusCode::OK);

    // Send valid APNS p8 Key
    let form = reqwest::multipart::Form::new()
        .text("apns_type", "token")
        .text("apns_topic", "app.test")
        .text("apns_key_id", env::var("ECHO_TEST_APNS_P8_KEY_ID").unwrap())
        .text(
            "apns_team_id",
            env::var("ECHO_TEST_APNS_P8_TEAM_ID").unwrap(),
        )
        .part(
            "apns_pkcs8_pem",
            reqwest::multipart::Part::text(env::var("ECHO_TEST_APNS_P8_PEM").unwrap())
                .file_name("apns.p8")
                .mime_str("text/plain")
                .expect("Error on passing multipart stream to the form request"),
        );
    let apns_update_result = client
        .post(format!(
            "http://{}/tenants/{}/apns",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Failed to call update tenant endpoint");
    assert_eq!(apns_update_result.status(), reqwest::StatusCode::OK);

    // Get tenant
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .send()
        .await
        .expect("Call failed");
    assert!(response.status().is_success());
    let response = response.json::<GetTenantResponse>().await.unwrap();
    println!("response: {response:?}");
    assert!(response
        .enabled_providers
        .contains(&PROVIDER_APNS.to_owned()));
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_delete(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register new tenant
    let client = reqwest::Client::new();
    let create_tenant_result = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Failed to create a new tenant");
    assert_eq!(create_tenant_result.status(), reqwest::StatusCode::OK);

    // Send valid APNS p8 Key
    let form = reqwest::multipart::Form::new()
        .text("apns_type", "token")
        .text("apns_topic", "app.test")
        .text("apns_key_id", env::var("ECHO_TEST_APNS_P8_KEY_ID").unwrap())
        .text(
            "apns_team_id",
            env::var("ECHO_TEST_APNS_P8_TEAM_ID").unwrap(),
        )
        .part(
            "apns_pkcs8_pem",
            reqwest::multipart::Part::text(env::var("ECHO_TEST_APNS_P8_PEM").unwrap())
                .file_name("apns.p8")
                .mime_str("text/plain")
                .expect("Error on passing multipart stream to the form request"),
        );

    let apns_update_result = client
        .post(format!(
            "http://{}/tenants/{}/apns",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Failed to call update tenant endpoint");
    assert_eq!(apns_update_result.status(), reqwest::StatusCode::OK);

    // Get tenant
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .send()
        .await
        .expect("Call failed");
    assert!(response.status().is_success());
    let response = response.json::<GetTenantResponse>().await.unwrap();
    println!("response: {response:?}");
    assert!(response
        .enabled_providers
        .contains(&PROVIDER_APNS.to_owned()));

    let apns_delete_result = client
        .delete(format!(
            "http://{}/tenants/{}/apns",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .send()
        .await
        .expect("Failed to call update tenant endpoint");
    assert_eq!(apns_delete_result.status(), reqwest::StatusCode::NO_CONTENT);

    // Get tenant
    let response = client
        .get(format!(
            "http://{}/tenants/{}",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .send()
        .await
        .expect("Call failed");
    assert!(response.status().is_success());
    let response = response.json::<GetTenantResponse>().await.unwrap();
    println!("response: {response:?}");
    assert!(response
        .enabled_providers
        .contains(&PROVIDER_APNS.to_owned()));
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_apns_bad_token(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register tenant
    let client = reqwest::Client::new();
    client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Call failed");

    // Send invalid API Key
    let form = reqwest::multipart::Form::new()
        .text("apns_pkcs8_pem", "invalid-pem")
        .text("apns_key_id", "invalid-key-id")
        .text("apns_team_id", "invalid-team-id");

    let response = client
        .post(format!(
            "http://{}/tenants/{}/apns",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(!response.status().is_success(), "Response was successful");
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_apns_bad_certificate(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register tenant
    let client = reqwest::Client::new();
    client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Call failed");

    // Send invalid API Key
    let form = reqwest::multipart::Form::new()
        .text("apns_certificate", "invalid-cert")
        .text("apns_certificate_password", "invalid-password");

    let response = client
        .post(format!(
            "http://{}/tenants/{}/apns",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(!response.status().is_success(), "Response was successful");
}
