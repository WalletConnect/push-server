use {
    crate::{
        error::{
            Error,
            Error::{BadFcmApiKey, InvalidMultipartBody},
        },
        increment_counter,
        state::AppState,
        stores::tenant::TenantFcmUpdateParams,
    },
    axum::{
        extract::{Multipart, Path, State},
        Json,
    },
    fcm::FcmError,
    serde::Serialize,
    std::sync::Arc,
};

pub struct FcmUpdateBody {
    api_key: String,
    /// Used to ensure that at least one value has changed
    value_changed_: bool,
}

#[derive(Serialize)]
pub struct UpdateTenantFcmResponse {
    success: bool,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    mut form_body: Multipart,
) -> Result<Json<UpdateTenantFcmResponse>, Error> {
    // -- check if tenant is real
    let _existing_tenant = state.tenant_store.get_tenant(&id).await?;

    // ---- retrieve body from form
    let mut body = FcmUpdateBody {
        api_key: Default::default(),
        value_changed_: false,
    };
    while let Some(field) = form_body.next_field().await? {
        let name = field.name().unwrap_or("unknown").to_string();
        let data = field.text().await?;

        if name.to_lowercase().as_str() == "api_key" {
            body.api_key = data;
            body.value_changed_ = true;
        };
    }
    if !body.value_changed_ {
        return Err(InvalidMultipartBody);
    }

    // ---- checks
    let fcm_api_key = body.api_key.clone();
    let mut test_message_builder = fcm::MessageBuilder::new(&fcm_api_key, "wc-notification-test");
    test_message_builder.dry_run(true);
    let test_message = test_message_builder.finalize();
    let test_notification = fcm::Client::new().send(test_message).await;
    match test_notification {
        Err(e) => match e {
            FcmError::Unauthorized => Err(BadFcmApiKey),
            _ => Ok(()),
        },
        Ok(_) => Ok(()),
    }?;

    // ---- handler
    let update_body = TenantFcmUpdateParams {
        fcm_api_key: body.api_key,
    };

    let new_tenant = state
        .tenant_store
        .update_tenant_fcm(&id, update_body)
        .await?;

    if new_tenant.suspended {
        // If suspended, it can be restored now because valid credentials have been
        // provided
        state.tenant_store.unsuspend_tenant(&new_tenant.id).await?;
    }

    increment_counter!(state.metrics, tenant_fcm_updates);

    Ok(Json(UpdateTenantFcmResponse { success: true }))
}
