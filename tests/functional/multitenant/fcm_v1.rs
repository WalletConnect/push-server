use {
    crate::{context::EchoServerContext, functional::multitenant::generate_random_tenant_id},
    echo_server::{
        handlers::{create_tenant::TenantRegisterBody, get_tenant::GetTenantResponse},
        providers::PROVIDER_FCM_V1,
    },
    std::env,
    test_context::test_context,
};

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_fcm_v1_valid(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register tenant
    let client = reqwest::Client::new();
    let register_response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Call failed");
    assert_eq!(register_response.status(), reqwest::StatusCode::OK);

    // Send valid API Key
    let credentials = env::var("ECHO_TEST_FCM_V1_CREDENTIALS").unwrap();
    let form = reqwest::multipart::Form::new().text("credentials", credentials);

    let response_fcm_update = client
        .post(format!(
            "http://{}/tenants/{}/fcm_v1",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response_fcm_update.status(), reqwest::StatusCode::OK);
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_enabled_providers(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register tenant
    let client = reqwest::Client::new();
    let register_response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Call failed");
    assert_eq!(register_response.status(), reqwest::StatusCode::OK);

    // Send valid API Key
    let credentials = env::var("ECHO_TEST_FCM_V1_CREDENTIALS").unwrap();
    let form = reqwest::multipart::Form::new().text("credentials", credentials);

    let response_fcm_update = client
        .post(format!(
            "http://{}/tenants/{}/fcm_v1",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response_fcm_update.status(), reqwest::StatusCode::OK);

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
        .contains(&PROVIDER_FCM_V1.to_owned()));
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_delete(ctx: &mut EchoServerContext) {
    let (tenant_id, jwt_token) = generate_random_tenant_id(&ctx.config.jwt_secret);

    // Register tenant
    let client = reqwest::Client::new();
    let register_response = client
        .post(format!("http://{}/tenants", ctx.server.public_addr))
        .bearer_auth(&jwt_token)
        .json(&TenantRegisterBody {
            id: tenant_id.clone(),
        })
        .send()
        .await
        .expect("Call failed");
    assert_eq!(register_response.status(), reqwest::StatusCode::OK);

    // Send valid API Key
    let credentials = env::var("ECHO_TEST_FCM_V1_CREDENTIALS").unwrap();
    let form = reqwest::multipart::Form::new().text("credentials", credentials);

    let response_fcm_update = client
        .post(format!(
            "http://{}/tenants/{}/fcm_v1",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response_fcm_update.status(), reqwest::StatusCode::OK);

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
        .contains(&PROVIDER_FCM_V1.to_owned()));

    let response_fcm_update = client
        .delete(format!(
            "http://{}/tenants/{}/fcm_v1",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .send()
        .await
        .expect("Call failed");
    assert_eq!(response_fcm_update.status(), reqwest::StatusCode::OK);

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
        .contains(&PROVIDER_FCM_V1.to_owned()));
}

#[test_context(EchoServerContext)]
#[tokio::test]
async fn tenant_update_fcm_v1_wrong_format(ctx: &mut EchoServerContext) {
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
    let form = reqwest::multipart::Form::new().text("credentials", "invalid-key");

    let response = client
        .post(format!(
            "http://{}/tenants/{}/fcm_v1",
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
async fn tenant_update_fcm_v1_invalidated(ctx: &mut EchoServerContext) {
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

    // Revoked key
    let credentials = r#"
        {
            "type": "service_account",
            "project_id": "push-server-tests-cc0f7",
            "private_key_id": "2a2d7c139c2d426be391d9003f2687a8f6f1fff2",
            "private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCeTJvksCrIC3pY\nOqHkIn2L5ZX11GO50pBE65U3Ik99Eh01puRPXitgdJZZIgcJEvSkDwcpq7ieuxe1\nSc35Le+STr8YM62Clp7KBIaTW4yiLxIN0PuLCgMBnTHKrcxwzCTOCog9rlYdGOj8\nnLd69FA9GUPiYvT+HToKRonm1N3kcAVLeUBj6z48qf4PxHAu0dpuGTfMcUUZfPZc\no6BXGrf2+eMTz0j7W6MCnpK1uVQu3rxjrvW2POgdHu31OvH9IAr8+CPuI3CTn/NF\n//uJOSYJmwysGyB/Fa6UqmCjd9u9VXLxOuIMOHXt54OegCuwb8/Y0GNZK4RBc4TS\nmAFramKvAgMBAAECggEAAupu5c8sUWpm/jdT+TJYS/FfgOSKCbVVLvRf6c6jH5yi\nqRl5AWL1zZQqdoAez1ce5nWPQsmE8neIUkLPwd05UCqWAxKuUqG9UUz9wQP0GueD\nn54Y9lCpwF63b8kiB75tKWiw03RPeAkqSFF08yevvPTwLbwumenvVJrWTJFOnQEa\noC4HCPXKNck2LDgnrfSynjYxhrICfXKwZ1Uhl8YYP5iSRf242nGjTyh2FwCM5nqx\nHbk3q/9RCb1VJoDBuSN7AcGm1mipqeJz5fTIGxAXcV0CJrkw3Betl0FYddj5Mc9I\n+koZf2HZbNkB4gF0SqnAakp3hNNqUOiFJPhavC4RoQKBgQDdbgmdUk+NyH57RDgx\nOSchoWgPVYLnQ9ZyDxXnxhzxdMPIDhKcCQpMEo7NR3SyzxcJNibDu3PeeAE5cw1L\n3nd2rFAdPUtSEnVgGxVN0hqAKWKfkCLtwOVznuilQVd1Px+4WJPJmsjQNWWWoGWE\nDcNEyg1lAhD4kiPHLrMPVea5CQKBgQC3A2jlY5h43CWU6Vmc2nEZQCEBYE7CShmc\np/6WvvxwDJxZ+qZR5xCPDwMV1Buw6N1LuGXoSOu4C9yP4qj3NrNVdiTmn/wI4OBl\nZQ/O5FcDHvVVO37cXrP2mKvMKBQfIbSu7oZrzlctyHj8KwWghXWpKai8sAk3dEyz\nNJKK9vzD9wKBgBnLAY+z0NSBMEqHjcweDjLarFZs7Yym2En8+949s41kvpGFIiHO\n48YsuzmqQyu498P47NcL9NlLPUlF35yg02WdeM+PHkD3KXkCbp7cBH49U+GmVos/\nVvr63bqyO8/KhJVirARl5VJrhePf1zNkrwRKTPkhHnz1+PjwtabpqLCRAoGANUL/\nxyqaGCpxoYnb86M7IQ8hy+W8ZhzsoUPe+v4wN2fkJOemedWWYxwKWNL3ECBlLwFG\nXzjBqTmCgjmD1RaNUITmrlvHHMpdZATqedrIW/cpjRmYjQfethiufub3HCxSCksO\nwdc2VfOvCix3IcVVfdrK6ccNl574J3tYXqsM2vcCgYBQKIHoaMfig1C7K6GVj04t\n7L5+hGoGkV3tbarWlN2f3fy+E8N8jNF0zcyy3DNy+nte8L525wV40rv0iUY7UUOD\nEYtL9cKJiAMVHBrmcxYYSKamDokUNYdbL6G/BiyAutDu5dvG/coVhY2DtlpUBuc6\nEZ5GAEY7kywLFZNd54/I2w==\n-----END PRIVATE KEY-----\n",
            "client_email": "firebase-adminsdk-eyh3f@push-server-tests-cc0f7.iam.gserviceaccount.com",
            "client_id": "102142178126793688289",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token",
            "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
            "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/firebase-adminsdk-eyh3f%40push-server-tests-cc0f7.iam.gserviceaccount.com",
            "universe_domain": "googleapis.com"
        }
    "#;
    let form = reqwest::multipart::Form::new().text("credentials", credentials);

    let response = client
        .post(format!(
            "http://{}/tenants/{}/fcm_v1",
            ctx.server.public_addr, tenant_id
        ))
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .expect("Call failed");

    assert!(!response.status().is_success(), "Response was successful");
}
