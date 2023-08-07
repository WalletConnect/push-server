use {
    crate::{
        handlers::{ErrorField, ErrorLocation, ResponseError},
        log::prelude::*,
        middleware::validate_signature::{SIGNATURE_HEADER_NAME, TIMESTAMP_HEADER_NAME},
        stores::StoreError,
    },
    axum::response::{IntoResponse, Response},
    hyper::StatusCode,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(is_variant_derive::IsVariant, Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Envy(#[from] envy::Error),

    #[error(transparent)]
    Trace(#[from] opentelemetry::trace::TraceError),

    #[error(transparent)]
    Metrics(#[from] opentelemetry::metrics::MetricsError),

    #[error(transparent)]
    Prometheus(#[from] prometheus_core::Error),

    #[error("invalid device token provided")]
    BadDeviceToken,

    #[error(transparent)]
    Apns(#[from] a2::Error),

    #[error("APNS Responded with error, {0}")]
    ApnsResponse(a2::ErrorReason),

    #[error(transparent)]
    Fcm(#[from] fcm::FcmError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Hex(#[from] hex::FromHexError),

    #[error(transparent)]
    Ed25519(#[from] ed25519_dalek::ed25519::Error),

    #[error(transparent)]
    HttpRequest(#[from] reqwest::Error),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Store(#[from] StoreError),

    #[error(transparent)]
    ToStr(#[from] axum::http::header::ToStrError),

    #[error("database migration failed: {0}")]
    DatabaseMigration(#[from] sqlx::migrate::MigrateError),

    #[error("{0} is an invalid push provider as it cannot be not found")]
    ProviderNotFound(String),

    #[error("{0} is an invalid push provider as it has not been enabled")]
    ProviderNotAvailable(String),

    #[error("the `{0}` field must not be empty")]
    EmptyField(String),

    #[error("a required environment variable cannot be found")]
    RequiredEnvNotFound,

    #[error("timestamp header cannot not found")]
    MissingTimestampHeader,

    #[error("signature header cannot not found")]
    MissingSignatureHeader,

    #[error("middleware T::from_request failed")]
    FromRequestError,

    #[error("middleware failed to parse body")]
    ToBytesError,

    #[error("neither signature or timestamp header cannot not found")]
    MissingAllSignatureHeader,

    #[error("single-tenant request made while echo server in multi-tenant mode")]
    MissingTenantId,

    #[error("multi-tenant request made while echo server in single-tenant mode")]
    IncludedTenantIdWhenNotNeeded,

    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("invalid tenant id: {0}")]
    InvalidTenantId(String),

    #[error("invalid options provided for {0}")]
    InvalidOptionsProvided(String),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    MultipartError(#[from] axum::extract::multipart::MultipartError),

    #[error("The provided multi-part body did not satisfy the requirements")]
    InvalidMultipartBody,

    #[error("invalid apns type: {0}")]
    InvalidApnsType(String),

    #[error("cannot get type when APNS is not configured")]
    NoApnsConfigured,

    #[error(
        "Encrypted push notification received without a topic, please ensure all required \
         parameters set"
    )]
    MissingTopic,

    #[error("client cannot be found")]
    ClientNotFound,

    #[error("this should not have occurred; used when case has been handled before")]
    InternalServerError,

    #[error(transparent)]
    JwtError(#[from] relay_rpc::auth::JwtVerificationError),

    #[error("the provided authentication does not authenticate the request")]
    InvalidAuthentication,

    #[error("GeoIpReader Error: {0}")]
    GeoIpReader(String),

    #[error("BatchCollector Error: {0}")]
    BatchCollector(String),

    #[error(transparent)]
    Registry(#[from] cerberus::registry::RegistryError),

    #[error("Invalid Project ID: {0}")]
    InvalidProjectId(String),

    #[error(transparent)]
    JWT(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    Parquet(#[from] gorgon::collectors::batch::BatchError<parquet::errors::ParquetError>),

    #[error("failed to load geoip database from s3")]
    GeoIpS3Failed,

    #[error("tenant id and client's registered tenant didn't match")]
    MissmatchedTenantId,

    #[error("invalid fcm api key")]
    BadFcmApiKey,

    #[error("invalid apns creds")]
    BadApnsCredentials,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("responding with error ({:?})", self);
        match self {
            Error::BadDeviceToken => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "invalid_token".to_string(),
                    message: "Provided device token is now invalid, the client id has been un-registered".to_string(),
                }
            ], vec![
                ErrorField {
                    field: "token".to_string(),
                    description: "Invalid device token".to_string(),
                    location: ErrorLocation::Body,
                }
            ]),
            Error::Apns(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "apns".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::ApnsResponse(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "apns_response".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::BadApnsCredentials => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "bad_apns_credentials".to_string(),
                    message: "Failed to validate the provided Certificate or Token".to_string(),
                }
            ], vec![]),
            Error::Fcm(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "fcm".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::BadFcmApiKey => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "bad_fcm_api_key".to_string(),
                    message: "The provided API Key was not valid".to_string(),
                }
            ], vec![
                ErrorField {
                    field: "api_key".to_string(),
                    description: "The provided API Key was not valid".to_string(),
                    location: ErrorLocation::Body,
                }
            ]),
            Error::Database(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "sqlx".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::Hex(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "from_hex".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::Ed25519(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "ed25519".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::HttpRequest(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "http_request".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::Base64Decode(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "base64_decode".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::Store(e) => match e {
                StoreError::Database(e) => crate::handlers::Response::new_failure(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    vec![ResponseError {
                        name: "sqlx".to_string(),
                        message: e.to_string(),
                    }],
                    vec![],
                ),
                StoreError::NotFound(entity, id) => crate::handlers::Response::new_failure(
                    StatusCode::NOT_FOUND,
                    vec![],
                    vec![ErrorField {
                        field: format!("{}.id", &entity),
                        description: format!("Cannot find {entity} with specified identifier {id}"),
                        location: ErrorLocation::Body, // TODO evaluate if correct location
                    }],
                ),
            },
            Error::ProviderNotFound(p) => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "provider_not_available".to_string(),
                    message: format!("The requested provider ({}) is not a valid provider", &p),
                }
            ], vec![
                ErrorField {
                    field: "provider".to_string(),
                    description: format!("The requested provider ({}) is not a valid provider", &p),
                    location: ErrorLocation::Body,
                }
            ]),
            Error::ProviderNotAvailable(p) => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "provider_not_available".to_string(),
                    message: format!("The requested provider ({}) is not currently available", &p),
                }
            ], vec![
                ErrorField {
                    field: "provider".to_string(),
                    description: format!("The requested provider ({}) is not currently available", &p),
                    location: ErrorLocation::Body,
                }
            ]),
            Error::MissingAllSignatureHeader => crate::handlers::Response::new_failure(StatusCode::UNAUTHORIZED, vec![
                ResponseError {
                    name: "webhook_validation_failed".to_string(),
                    message: "Failed to validate webhook, please ensure that all required headers are provided.".to_string(),
                }
            ], vec![
                ErrorField {
                    field: SIGNATURE_HEADER_NAME.to_string(),
                    description: "Missing signature".to_string(),
                    location: ErrorLocation::Header,
                },
                ErrorField {
                    field: TIMESTAMP_HEADER_NAME.to_string(),
                    description: "Missing timestamp".to_string(),
                    location: ErrorLocation::Header,
                },
            ]),
            Error::MissingSignatureHeader => crate::handlers::Response::new_failure(StatusCode::UNAUTHORIZED, vec![
                ResponseError {
                    name: "webhook_validation_failed".to_string(),
                    message: "Failed to validate webhook, please ensure that all required headers are provided.".to_string(),
                }
            ], vec![
                ErrorField {
                    field: SIGNATURE_HEADER_NAME.to_string(),
                    description: "Missing signature".to_string(),
                    location: ErrorLocation::Header,
                }
            ]),
            Error::MissingTimestampHeader => crate::handlers::Response::new_failure(StatusCode::UNAUTHORIZED, vec![
                ResponseError {
                    name: "webhook_validation_failed".to_string(),
                    message: "Failed to validate webhook, please ensure that all required headers are provided.".to_string(),
                }
            ], vec![
                ErrorField {
                    field: TIMESTAMP_HEADER_NAME.to_string(),
                    description: "Missing timestamp".to_string(),
                    location: ErrorLocation::Header,
                }
            ]),
            Error::InvalidTenantId(id) => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "tenant".to_string(),
                    message: format!("The provided Tenant ID, {}, is invalid. Please ensure it's valid and the url is in the format /:tenant_id/...path", &id),
                }
            ], vec![
                ErrorField {
                    field: "tenant_id".to_string(),
                    description: format!("Invalid Tenant ID, {}", &id),
                    location: ErrorLocation::Path,
                }
            ]),
            Error::MissingTenantId => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                vec![ResponseError {
                    name: "tenancy-mode".to_string(),
                    message: "single-tenant request made while echo server in multi-tenant mode".to_string(),
                }],
                vec![],
            ),
            Error::IncludedTenantIdWhenNotNeeded => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                vec![ResponseError {
                    name: "tenancy-mode".to_string(),
                    message: "multi-tenant request made while echo server in single-tenant mode".to_string(),
                }],
                vec![],
            ),
            Error::MissingTopic => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                vec![ResponseError {
                    name: "topic".to_string(),
                    message: "encrypted push notifications require topic to be set".to_string(),
                }],
                vec![],
            ),
            // If the client cannot be found we gracefully handle this
            Error::ClientNotFound => crate::handlers::Response::new_success(StatusCode::ACCEPTED),
            Error::JwtError(_) | Error::InvalidAuthentication => crate::handlers::Response::new_failure(
                StatusCode::UNAUTHORIZED,
                vec![ResponseError {
                    name: "authentication".to_string(),
                    message: "the provided authentication is not sufficient".to_string(),
                }],
                vec![
                    ErrorField {
                        field: axum::http::header::AUTHORIZATION.to_string(),
                        description: "invalid authorization token".to_string(),
                        location: ErrorLocation::Header,
                    }
                ],
            ),
            Error::InvalidProjectId(id) => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                vec![ResponseError {
                    name: "project_id".to_string(),
                    message: format!("the provided project id ({}) is not valid", id),
                }],
                vec![
                    ErrorField {
                        field: "id".to_string(),
                        description: "invalid project id".to_string(),
                        location: ErrorLocation::Body,
                    }
                ],
            ),
            Error::Io(_) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "io".to_string(),
                    message: "failed to perform io task, this should not have occurred, please report at: https://github.com/walletconnect/echo-server".to_string(),
                },
            ], vec![]),
            Error::Json(_) => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "json".to_string(),
                    message: "received invalid json payload, please ensure all fields that contain json are valid".to_string(),
                },
            ], vec![]),
            Error::ToStr(_) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "header_decode".to_string(),
                    message: "failed to decode header using axum, please try again or report error at: https://github.com/walletconnect/echo-server".to_string(),
                },
            ], vec![]),
            Error::EmptyField(f) => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "field".to_string(),
                    message: "field is missing from request".to_string(),
                },
            ], vec![
                ErrorField {
                    field: f,
                    description: "missing from request".to_string(),
                    // Note (Harry Bairstow): Currently only used in body
                    location: ErrorLocation::Body,
                }
            ]),
            Error::FromRequestError => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "unknown".to_string(),
                    message: "unknown error when forwarding request without signature validation, please report this at: https://github.com/walletconnect/echo-server".to_string(),
                },
            ], vec![]),
            Error::ToBytesError => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "decode".to_string(),
                    message: "failed to decode body as bytes, please ensure your JSON body is encoded properly".to_string(),
                },
            ], vec![]),
            // Figure out
            // TODO: Error::InvalidOptionsProvided(_) => {}
            Error::FromUtf8Error(_) => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "decode".to_string(),
                    message: "failed to decode body as Utf8, please ensure your JSON body is encoded properly".to_string(),
                },
            ], vec![]),
            Error::MultipartError(e) => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "body".to_string(),
                    message: "failed to decode the multipart body".to_string(),
                },
                ResponseError {
                    name: "multipart".to_string(),
                    message: format!("{:?}", e),
                },
            ], vec![]),
            Error::InvalidMultipartBody => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "body".to_string(),
                    message: "multipart body did not conform to specification".to_string(),
                },
            ], vec![]),
            Error::InvalidApnsType(t) => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "decoding_error".to_string(),
                    message: format!("failed to decode apns type, \"{}\" is invalid", t),
                },
            ], vec![
                ErrorField {
                    field: "type".to_string(),
                    description: "apns push type decoding failed".to_string(),
                    location: ErrorLocation::Unknown,
                }
            ]),
            Error::InternalServerError => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "unknown_error".to_string(),
                    message: "This error should not have occurred. Please file an issue at: https://github.com/walletconnect/echo-server".to_string(),
                },
            ], vec![]),
            Error::GeoIpReader(_) | Error::BatchCollector(_) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "o11y".to_string(),
                    message: "Internal error monitoring the request".to_string(),
                },
            ], vec![]),
            Error::Registry(_) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "internal_api_failed".to_string(),
                    message: "Please check https://status.walletconnect.com as an internal API failed to resolve this request".to_string(),
                },
            ], vec![]),
            Error::JWT(_) => crate::handlers::Response::new_failure(StatusCode::UNAUTHORIZED, vec![
                ResponseError {
                    name: "invalid_auth".to_string(),
                    message: "JWT Authentication Failed".to_string(),
                },
            ], vec![]),
            Error::MissmatchedTenantId => crate::handlers::Response::new_failure(StatusCode::BAD_REQUEST, vec![
                ResponseError {
                    name: "missmatched_identifiers".to_string(),
                    message: "The requested tenant doesn't have this client registered".to_string(),
                },
            ], vec![
                ErrorField {
                    field: "tenant_id".to_string(),
                    description: "doesn't match registered id".to_string(),
                    location: ErrorLocation::Path,
                },
                ErrorField {
                    field: "id".to_string(),
                    description: "doesn't match registered id".to_string(),
                    location: ErrorLocation::Path,
                }
            ]),
            e => {
                warn!("Error does not have response clause, {:?}", e);

                crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                    ResponseError {
                        name: "unknown_error".to_string(),
                        message: "This error should not have occurred. Please file an issue at: https://github.com/walletconnect/echo-server".to_string(),
                    },
                    ResponseError {
                        name: "dbg".to_string(),
                        message: format!("{e:?}"),
                    },
                ], vec![])
            }
        }.into_response()
    }
}
