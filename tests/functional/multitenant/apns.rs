use {
    crate::context::EchoServerContext, echo_server::handlers::create_tenant::TenantRegisterBody,
    random_string::generate, test_context::test_context,
};

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
