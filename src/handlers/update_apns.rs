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
    pub apns_topic: Option<String>,

    pub apns_certificate: Option<String>,
    pub apns_certificate_password: Option<String>,

    pub apns_pkcs8_pem: Option<String>,
    pub apns_key_id: Option<String>,
    pub apns_team_id: Option<String>,
}

impl ApnsUpdateBody {
    pub fn validate(&self) -> bool {
        // Match cases when the input is not valid and return false.
        // Input is valid if certificate and certificate_password is included for
        // updates. topic is required for new tenants

        match (
            &self.apns_topic,
            &self.apns_certificate,
            &self.apns_certificate_password,
            &self.apns_pkcs8_pem,
            &self.apns_key_id,
            &self.apns_team_id,
        ) {
            // Update Topic
            (Some(_), None, None, None, None, None) => true,
            // Update Certificate
            (Some(_), Some(_), Some(_), None, None, None) => true,
            (None, Some(_), Some(_), None, None, None) => true,
            // Update Token
            (Some(_), None, None, Some(_), Some(_), Some(_)) => true,
            (None, None, None, Some(_), Some(_), Some(_)) => true,
            // All other cases are invalid
            _ => false,
        }
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

        apns_pkcs8_pem: None,
        apns_key_id: None,
        apns_team_id: None,
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
                let encoded_certificate = base64::engine::general_purpose::STANDARD.encode(&data);
                body.apns_certificate = Some(encoded_certificate);
            }
            "apns_certificate_password" => {
                body.apns_certificate_password = Some(field.text().await?);
            }
            "apns_pkcs8_pem" => {
                let data = field.bytes().await?;
                let encoded_pem = base64::engine::general_purpose::STANDARD.encode(&data);
                body.apns_pkcs8_pem = Some(encoded_pem);
            }
            "apns_key_id" => {
                body.apns_key_id = Some(field.text().await?);
            }
            "apns_team_id" => {
                body.apns_team_id = Some(field.text().await?);
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
        apns_pkcs8_pem: existing_tenant.apns_pkcs8_pem,
        apns_key_id: existing_tenant.apns_key_id,
        apns_team_id: existing_tenant.apns_team_id,
    };

    if let Some(topic) = body.apns_topic {
        update_body.apns_topic = Some(topic);
    }

    if let Some(cert) = body.apns_certificate {
        update_body.apns_certificate = Some(cert);
    }

    if let Some(password) = body.apns_certificate_password {
        update_body.apns_certificate_password = Some(password);
    }

    if let Some(pem) = body.apns_pkcs8_pem {
        update_body.apns_pkcs8_pem = Some(pem);
    }

    if let Some(key) = body.apns_key_id {
        update_body.apns_key_id = Some(key);
    }

    if let Some(team) = body.apns_team_id {
        update_body.apns_team_id = Some(team);
    }

    let _new_tenant = state.tenant_store.update_tenant(update_body).await?;

    Ok(Json(UpdateTenantApnsResponse { success: true }))
}
