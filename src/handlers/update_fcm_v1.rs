use {
    crate::{
        error::{Error, Error::InvalidMultipartBody},
        handlers::validate_tenant_request,
        increment_counter,
        state::AppState,
        stores::tenant::TenantFcmV1UpdateParams,
    },
    axum::{
        extract::{Multipart, Path, State},
        http::HeaderMap,
        Json,
    },
    serde::Serialize,
    std::sync::Arc,
    tracing::{error, instrument},
};

pub struct FcmV1UpdateBody {
    credentials: String,
    /// Used to ensure that at least one value has changed
    value_changed_: bool,
}

#[derive(Serialize)]
pub struct UpdateTenantFcmV1Response {
    success: bool,
}

#[instrument(skip_all, name = "update_fcm_v1_handler")]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
    mut form_body: Multipart,
) -> Result<Json<UpdateTenantFcmV1Response>, Error> {
    // -- check if tenant is real
    let _existing_tenant = state.tenant_store.get_tenant(&id).await?;

    // JWT token verification
    #[cfg(feature = "cloud")]
    let jwt_verification_result = validate_tenant_request(
        &state.registry_client,
        &state.gotrue_client,
        &headers,
        id.clone(),
        None,
    )
    .await;

    #[cfg(not(feature = "cloud"))]
    let jwt_verification_result = validate_tenant_request(&state.gotrue_client, &headers);

    if let Err(e) = jwt_verification_result {
        error!(
            tenant_id = %id,
            err = ?e,
            "JWT verification failed"
        );
        return Err(e);
    }

    // ---- retrieve body from form
    let mut body = FcmV1UpdateBody {
        credentials: Default::default(),
        value_changed_: false,
    };
    while let Some(field) = form_body.next_field().await? {
        let name = field.name().unwrap_or("unknown").to_string();
        let data = field.text().await?;

        if name.to_lowercase().as_str() == "credentials" {
            body.credentials = data;
            body.value_changed_ = true;
        };
    }
    if !body.value_changed_ {
        return Err(InvalidMultipartBody);
    }

    // ---- checks
    // TODO
    // let fcm_credentials = body.credentials.clone();
    // let mut test_message_builder = fcm_v1::Message::new(&fcm_credentials, "wc-notification-test");
    // test_message_builder.dry_run(true);
    // let test_message = test_message_builder.finalize();
    // let test_notification = fcm::Client::new().send(test_message).await;
    // match test_notification {
    //     Err(e) => match e {
    //         FcmError::Unauthorized => Err(BadFcmApiKey),
    //         _ => Ok(()),
    //     },
    //     Ok(_) => Ok(()),
    // }?;

    // ---- handler
    let update_body = TenantFcmV1UpdateParams {
        fcm_v1_credentials: body.credentials,
    };

    let new_tenant = state
        .tenant_store
        .update_tenant_fcm_v1(&id, update_body)
        .await?;

    if new_tenant.suspended {
        // If suspended, it can be restored now because valid credentials have been
        // provided
        state.tenant_store.unsuspend_tenant(&new_tenant.id).await?;
    }

    increment_counter!(state.metrics, tenant_fcm_v1_updates);

    Ok(Json(UpdateTenantFcmV1Response { success: true }))
}
