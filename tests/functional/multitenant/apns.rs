use {
    crate::context::EchoServerContext,
    echo_server::handlers::{create_tenant::TenantRegisterBody, get_tenant::GetTenantResponse},
    jsonwebtoken::{encode, EncodingKey, Header},
    random_string::generate,
    serde::Serialize,
    std::time::SystemTime,
    test_context::test_context,
    uuid::Uuid,
};

/// Struct to hold claims for JWT validation
#[derive(Serialize)]
struct ClaimsForValidation {
    sub: String,
    aud: String,
    role: String,
    exp: usize,
}

// #[test_context(EchoServerContext)]
// #[tokio::test]
// async fn tenant_update_apns(ctx: &mut EchoServerContext) {
//     let charset = "1234567890";
//     let random_tenant_id = generate(12, charset);
//     let payload = TenantRegisterBody {
//         id: random_tenant_id.clone(),
//     };
//
//     // Register tenant
//     let client = reqwest::Client::new();
//     let response = client
//         .post(format!("http://{}/tenants", ctx.server.public_addr))
//         .json(&payload)
//         .send()
//         .await
//         .expect("Call failed");
//
//     // Send valid token/cert
//     // TODO figure out how to get valid creds into test!
//     let api_key = env!("ECHO_TEST_FCM_KEY");
//     let form = reqwest::multipart::Form::new().text("api_key", api_key);
//
//     let response = client
//         .post(format!(
//             "http://{}/tenants/{}/apns",
//             ctx.server.public_addr, &random_tenant_id
//         ))
//         .multipart(form)
//         .send()
//         .await
//         .expect("Call failed");
//
//     assert!(
//         response.status().is_success(),
//         "Response was not successful"
//     );
// }

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_apns_valid_token(ctx: &mut EchoServerContext) {
    let tenant_id = Uuid::new_v4().to_string();
    let payload = TenantRegisterBody {
        id: tenant_id.clone(),
    };
    let unix_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let token_claims = ClaimsForValidation {
        sub: tenant_id.clone(),
        aud: "authenticated".to_string(),
        role: "authenticated".to_string(),
        exp: unix_timestamp + 60 * 60, // Add an hour for expiration
    };
    let jwt_token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(ctx.config.jwt_secret.as_bytes()),
    )
    .expect("Failed to encode jwt token");

    // Register new tenant
    let client = reqwest::Client::new();
    let create_tenant_result = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .header("AUTHORIZATION", jwt_token.clone())
        .json(&payload)
        .send()
        .await
        .expect("Failed to create a new tenant");
    assert_eq!(create_tenant_result.status(), reqwest::StatusCode::OK);

    // Send valid APNS p8 Key
    let form = reqwest::multipart::Form::new()
        .text("apns_type", "token")
        .text("apns_topic", "app.test")
        .text("apns_key_id", env!("ECHO_TEST_APNS_P8_KEY_ID"))
        .text("apns_team_id", env!("ECHO_TEST_APNS_P8_TEAM_ID"))
        .part(
            "apns_pkcs8_pem",
            reqwest::multipart::Part::text(env!("ECHO_TEST_APNS_P8_PEM"))
                .file_name("apns.p8")
                .mime_str("text/plain")
                .expect("Error on passing multipart stream to the form request"),
        );
    let apns_update_result = client
        .post(format!(
            "http://{}/tenants/{}/apns",
            ctx.server.public_addr, &tenant_id
        ))
        .multipart(form)
        .send()
        .await
        .expect("Failed to call update tenant endpoint");
    assert_eq!(apns_update_result.status(), reqwest::StatusCode::OK);
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_apns_bad_token(ctx: &mut EchoServerContext) {
    let charset = "1234567890";
    let random_tenant_id = generate(12, charset);
    let payload = TenantRegisterBody {
        id: random_tenant_id.clone(),
    };

    // Register tenant
    let client = reqwest::Client::new();
    client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .json(&payload)
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
            ctx.server.public_addr, &random_tenant_id
        ))
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(!response.status().is_success(), "Response was successful");
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_apns_bad_certificate(ctx: &mut EchoServerContext) {
    let charset = "1234567890";
    let random_tenant_id = generate(12, charset);
    let payload = TenantRegisterBody {
        id: random_tenant_id.clone(),
    };

    // Register tenant
    let client = reqwest::Client::new();
    client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .json(&payload)
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
            ctx.server.public_addr, &random_tenant_id
        ))
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(!response.status().is_success(), "Response was successful");
}
