use {
    crate::{
        error::{Error, Error::InvalidMultipartBody},
        state::AppState,
        stores::tenant::TenantUpdateParams,
    },
    axum::{
        extract::{Multipart, Path, State},
        Json,
    },
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

    // ---- handler
    let existing_tenant = state.tenant_store.get_tenant(&id).await?;
    let update_body = TenantUpdateParams {
        id: Some(existing_tenant.id),
        fcm_api_key: Some(body.api_key),
        apns_topic: existing_tenant.apns_topic,
        apns_certificate: existing_tenant.apns_certificate,
        apns_certificate_password: existing_tenant.apns_certificate_password,
    };

    let _new_tenant = state.tenant_store.update_tenant(update_body).await?;

    Ok(Json(UpdateTenantFcmResponse { success: true }))
}
