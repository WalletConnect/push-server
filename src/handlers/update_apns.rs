use {
    crate::{
        error::{
            Error,
            Error::{InternalServerError, InvalidMultipartBody},
        },
        increment_counter,
        state::AppState,
        stores::tenant::{ApnsType, TenantApnsUpdateParams},
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
    pub fn validate(&self) -> Result<Option<ApnsType>, Error> {
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
            (Some(_), None, None, None, None, None) => Ok(None),
            // Update Certificate
            (Some(_), Some(_), Some(_), None, None, None) => Ok(Some(ApnsType::Certificate)),
            (None, Some(_), Some(_), None, None, None) => Ok(Some(ApnsType::Certificate)),
            // Update Token
            (Some(_), None, None, Some(_), Some(_), Some(_)) => Ok(Some(ApnsType::Token)),
            (None, None, None, Some(_), Some(_), Some(_)) => Ok(Some(ApnsType::Token)),
            // All other cases are invalid
            _ => Err(InvalidMultipartBody),
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
    // Ensure tenant real
    let existing_tenant = state.tenant_store.get_tenant(&id).await?;

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

    let apns_type = body.validate()?;

    if apns_type.is_none() {
        // Just update topic
        let update_body = TenantApnsUpdateParams {
            apns_topic: body.apns_topic,
            apns_type: existing_tenant.apns_type,
            apns_certificate: existing_tenant.apns_certificate,
            apns_certificate_password: existing_tenant.apns_certificate_password,
            apns_pkcs8_pem: existing_tenant.apns_pkcs8_pem,
            apns_key_id: existing_tenant.apns_key_id,
            apns_team_id: existing_tenant.apns_team_id,
        };

        let _new_tenant = state
            .tenant_store
            .update_tenant_apns(&id, update_body)
            .await?;

        increment_counter!(state.metrics, tenant_apns_updates);

        return Ok(Json(UpdateTenantApnsResponse { success: true }));
    }

    // ---- handler
    let update_body = match apns_type {
        None => Err(InternalServerError),
        Some(t) => Ok(match t {
            ApnsType::Certificate => TenantApnsUpdateParams {
                apns_type: Some(ApnsType::Certificate),
                apns_topic: existing_tenant.apns_topic,
                apns_certificate: body.apns_certificate,
                apns_certificate_password: body.apns_certificate_password,
                apns_pkcs8_pem: None,
                apns_key_id: None,
                apns_team_id: None,
            },
            ApnsType::Token => TenantApnsUpdateParams {
                apns_type: Some(ApnsType::Token),
                apns_topic: existing_tenant.apns_topic,
                apns_certificate: None,
                apns_certificate_password: None,
                apns_pkcs8_pem: body.apns_pkcs8_pem,
                apns_key_id: body.apns_key_id,
                apns_team_id: body.apns_team_id,
            },
        }),
    }?;

    let _new_tenant = state
        .tenant_store
        .update_tenant_apns(&id, update_body)
        .await?;

    increment_counter!(state.metrics, tenant_apns_updates);

    Ok(Json(UpdateTenantApnsResponse { success: true }))
}
