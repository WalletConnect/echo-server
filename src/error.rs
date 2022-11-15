use crate::handlers::{ErrorField, ErrorLocation, ResponseError};
use crate::middleware::validate_signature::{SIGNATURE_HEADER_NAME, TIMESTAMP_HEADER_NAME};
use crate::stores::StoreError;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Envy(#[from] envy::Error),

    #[error(transparent)]
    Trace(#[from] opentelemetry::trace::TraceError),

    #[error(transparent)]
    Metrics(#[from] opentelemetry::metrics::MetricsError),

    #[error(transparent)]
    Apns(#[from] a2::Error),

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
    Store(#[from] StoreError),

    #[error("database migration failed: {0}")]
    DatabaseMigration(#[from] sqlx::migrate::MigrateError),

    #[error("{0} is an invalid push provider as it cannot be not found")]
    ProviderNotFound(String),

    #[error("{0} is an invalid push provider as it has not been enabled")]
    ProviderNotAvailable(String),

    #[error("the `{0}` field must not be empty")]
    EmptyField(String),

    #[error("a client is already registered with the provided identifier")]
    ClientAlreadyRegistered,

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

    #[error("Failed to unwrap a mutex as it was poisoned")]
    MutexPoison,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Apns(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "apns".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
            Error::Fcm(e) => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "fcm".to_string(),
                    message: e.to_string(),
                }
            ], vec![]),
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
            Error::ClientAlreadyRegistered => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                vec![],
                vec![
                    ErrorField {
                        field: "client_id".to_string(),
                        description: "Client already registered with the provided identifier".to_string(),
                        location: ErrorLocation::Body,
                    }
                ],
            ),
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
                        description: format!("Cannot find {} with specified identifier {}", entity, id),
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
                    location: ErrorLocation::Body
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
                    location: ErrorLocation::Body
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
                    location: ErrorLocation::Header
                },
                ErrorField {
                    field: TIMESTAMP_HEADER_NAME.to_string(),
                    description: "Missing timestamp".to_string(),
                    location: ErrorLocation::Header
                }
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
                    location: ErrorLocation::Header
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
                    location: ErrorLocation::Header
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
                    location: ErrorLocation::Path
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
            _ => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "unknown_error".to_string(),
                    message: "This error should not have occurred. Please file an issue at: https://github.com/walletconnect/echo-server".to_string(),
                }
            ], vec![])
        }.into_response()
    }
}
