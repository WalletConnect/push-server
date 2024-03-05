#[cfg(feature = "cloud")]
use {
    crate::error::Error::InvalidProjectId,
    async_recursion::async_recursion,
    cerberus::{
        project::ProjectData,
        registry::{RegistryClient, RegistryHttpClient},
    },
};
use {
    crate::{
        error::{Error::InvalidAuthentication, Result},
        supabase::GoTrueClient,
    },
    axum::{
        http::{header::AUTHORIZATION, HeaderMap},
        response::IntoResponse,
        Json,
    },
    hyper::StatusCode,
    relay_rpc::{
        domain::ClientId,
        jwt::{JwtBasicClaims, VerifyableClaims},
    },
    serde_json::{json, Value},
    std::{collections::HashSet, string::ToString},
    tracing::{debug, instrument},
};

// Push
pub mod delete_client;
pub mod metrics;
pub mod push_message;
pub mod register_client;
#[cfg(not(feature = "multitenant"))]
pub mod single_tenant_wrappers;
// Tenant Management
#[cfg(feature = "multitenant")]
pub mod create_tenant;
#[cfg(feature = "multitenant")]
pub mod delete_tenant;
#[cfg(feature = "multitenant")]
pub mod get_tenant;
pub mod health;
#[cfg(feature = "multitenant")]
pub mod update_apns;
#[cfg(feature = "multitenant")]
pub mod update_fcm;

pub const DECENTRALIZED_IDENTIFIER_PREFIX: &str = "did:key:";

#[instrument(skip_all)]
pub fn authenticate_client<F>(headers: HeaderMap, aud: &str, check: F) -> Result<bool>
where
    F: FnOnce(Option<ClientId>) -> bool,
{
    return if let Some(auth_header) = headers.get(axum::http::header::AUTHORIZATION) {
        let header_str = auth_header.to_str()?;

        let claims = JwtBasicClaims::try_from_str(header_str).map_err(|e| {
            debug!("Invalid claims: {:?}", e);
            e
        })?;
        claims
            .verify_basic(&HashSet::from([aud.to_string()]), None)
            .map_err(|e| {
                debug!("Failed to verify_basic: {:?}", e);
                e
            })?;
        let client_id: ClientId = claims.iss.into();
        Ok(check(Some(client_id)))
    } else {
        // Note: Authentication is not required right now to ensure that this is a
        // non-breaking change, eventually it will be required and this should default
        // to returning `Err(MissingAuthentication)` or `Err(InvalidAuthentication)`
        Ok(true)
    };
}

#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorLocation {
    Body,
    // Note (Harry): Spec supports this but it currently isn't used
    // Query,
    Header,
    Path,
    Unknown,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(serde::Serialize)]
pub struct ErrorField {
    pub field: String,
    pub description: String,
    pub location: ErrorLocation,
}

#[derive(serde::Serialize)]
pub struct ResponseError {
    pub name: String,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct Response {
    pub status: ResponseStatus,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub errors: Option<Vec<ResponseError>>,
    pub fields: Option<Vec<ErrorField>>,
}

impl Response {
    pub fn new_success(status: StatusCode) -> Self {
        Response {
            status: ResponseStatus::Success,
            status_code: status,
            errors: None,
            fields: None,
        }
    }

    pub fn new_failure(
        status: StatusCode,
        errors: Vec<ResponseError>,
        fields: Vec<ErrorField>,
    ) -> Self {
        Response {
            status: ResponseStatus::Failure,
            status_code: status,
            errors: Some(errors),
            fields: Some(fields),
        }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code;
        let json: Json<Value> = self.into();

        (status, json).into_response()
    }
}

impl From<Response> for Json<Value> {
    fn from(value: Response) -> Self {
        Json(json!(value))
    }
}

impl Default for Response {
    fn default() -> Self {
        Response::new_success(StatusCode::OK)
    }
}

#[async_recursion]
#[cfg(feature = "cloud")]
#[instrument(skip_all, fields(project_id = %project_id, project = ?project))]
pub async fn validate_tenant_request(
    registry_client: &RegistryHttpClient,
    gotrue_client: &GoTrueClient,
    headers: &HeaderMap,
    project_id: String,
    project: Option<ProjectData>,
) -> Result<bool> {
    if let Some(project) = project {
        if let Some(token_value) = headers.get(AUTHORIZATION) {
            Ok(match gotrue_client
                .is_valid_token(token_value.to_str()?.to_string().replace("Bearer ", ""))
            {
                Ok(token_data) => {
                    #[cfg(feature = "cloud")]
                    let valid_token = token_data.claims.sub == project.creator;

                    #[cfg(not(feature = "cloud"))]
                    let valid_token = true;

                    if !valid_token {
                        Err(InvalidAuthentication)
                    } else {
                        Ok(true)
                    }
                }
                Err(_) => Err(InvalidAuthentication),
            }?)
        } else {
            Err(InvalidAuthentication)
        }
    } else if let Some(project_fetched) = registry_client.project_data(&project_id).await? {
        validate_tenant_request(
            registry_client,
            gotrue_client,
            headers,
            project_id,
            Some(project_fetched),
        )
        .await
    } else {
        Err(InvalidProjectId(project_id.to_string()))
    }
}

#[cfg(not(feature = "cloud"))]
#[instrument(skip_all)]
pub fn validate_tenant_request(gotrue_client: &GoTrueClient, headers: &HeaderMap) -> Result<bool> {
    if let Some(token_data) = headers.get(AUTHORIZATION) {
        if gotrue_client
            .is_valid_token(token_data.to_str()?.to_string().replace("Bearer ", ""))
            .is_ok()
        {
            Ok(true)
        } else {
            Err(InvalidAuthentication)
        }
    } else {
        Err(InvalidAuthentication)
    }
}
