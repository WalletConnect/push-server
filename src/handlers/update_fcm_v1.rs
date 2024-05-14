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
    tracing::{debug, error, instrument},
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
    // JWT token verification
    #[cfg(feature = "cloud")]
    let jwt_verification_result =
        validate_tenant_request(&state.jwt_validation_client, &headers, &id).await;

    // -- check if tenant is real
    let _existing_tenant = state.tenant_store.get_tenant(&id).await?;

    #[cfg(not(feature = "cloud"))]
    let jwt_verification_result = validate_tenant_request(&state.jwt_validation_client, &headers);

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

    // Client will validate the key on startup
    fcm_v1::Client::from_key(
        serde_json::from_str(&body.credentials).map_err(Error::FcmV1InvalidServiceAccountKey)?,
    )
    .await
    .map_err(|e| {
        debug!("Failed credential validation: {e}");
        Error::BadFcmV1Credentials
    })?;

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
