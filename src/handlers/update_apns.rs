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
    base64::Engine,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[derive(Deserialize)]
pub struct ApnsUpdateBody {
    apns_topic: Option<String>,
    apns_certificate: Option<String>,
    apns_certificate_password: Option<String>,
}

impl ApnsUpdateBody {
    pub fn validate(&self) -> bool {
        // Match cases when the input is not valid and return false.
        // Input is valid if certificate and certificate_password is included for
        // updates. topic is required for new tenants
        if self.apns_certificate.is_some() && self.apns_certificate_password.is_none() {
            return false; // New certificate without new password
        }

        if self.apns_certificate.is_none() && self.apns_certificate_password.is_some() {
            return false; // New password without certificate
        }

        true
    }
}

#[derive(Serialize)]
pub struct UpdateTenantApnsResponse {
    success: bool,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    mut form_body: Multipart,
) -> Result<Json<UpdateTenantApnsResponse>, Error> {
    // ---- retrieve body from form
    let mut body = ApnsUpdateBody {
        apns_topic: None,
        apns_certificate: None,
        apns_certificate_password: None,
    };
    while let Some(field) = form_body.next_field().await? {
        let name = field.name().unwrap_or("unknown").to_string();

        // Check the lowercase name against list of known names for struct
        match name.to_lowercase().as_str() {
            "apns_topic" => {
                body.apns_topic = Some(field.text().await?);
            }
            "apns_certificate" => {
                let data = field.bytes().await?;
                let encoded_certificate = base64::engine::general_purpose::STANDARD.encode(&data.to_vec());
                body.apns_certificate = Some(encoded_certificate);
            }
            "apns_certificate_password" => {
                body.apns_certificate_password = Some(field.text().await?);
            }
            _ => {
                // Unknown field, ignored
            }
        };
    }

    if !body.validate() {
        return Err(InvalidMultipartBody);
    }

    // ---- handler
    let existing_tenant = state.tenant_store.get_tenant(&id).await?;

    let mut update_body = TenantUpdateParams {
        id: Some(existing_tenant.id),
        fcm_api_key: existing_tenant.fcm_api_key,
        apns_topic: existing_tenant.apns_topic,
        apns_certificate: existing_tenant.apns_certificate,
        apns_certificate_password: existing_tenant.apns_certificate_password,
    };

    if let Some(cert) = body.apns_certificate {
        update_body.apns_certificate = Some(cert);
    }

    if let Some(password) = body.apns_certificate_password {
        update_body.apns_certificate_password = Some(password);
    }

    if let Some(topic) = body.apns_topic {
        update_body.apns_topic = Some(topic);
    }

    let _new_tenant = state.tenant_store.update_tenant(update_body).await?;

    Ok(Json(UpdateTenantApnsResponse { success: true }))
}
