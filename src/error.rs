use reqwest::header::InvalidHeaderValue;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

use crate::AuthenticatorError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorWrapper {
    pub error: ApiErrorMessage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorMessage {
    pub code: u32,
    pub message: String,
    pub missing: Option<Vec<HashMap<String, String>>>,
    pub duplicated: Option<Vec<HashMap<String, String>>>,
}

#[derive(Debug, Default)]
pub struct CdfApiError {
    pub code: u32,
    pub message: String,
    pub missing: Option<Vec<HashMap<String, String>>>,
    pub duplicated: Option<Vec<HashMap<String, String>>>,
    pub request_id: Option<String>,
}

impl fmt::Display for CdfApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}. RequestId: {}",
            &self.code,
            &self.message,
            self.request_id.as_deref().unwrap_or("")
        )
    }
}

impl CdfApiError {
    pub fn new(raw: ApiErrorMessage, request_id: Option<String>) -> Self {
        CdfApiError {
            code: raw.code,
            message: raw.message,
            missing: raw.missing,
            duplicated: raw.duplicated,
            request_id,
        }
    }
}

#[derive(Debug, Error)]
pub struct Error {
    pub kind: Kind,
}

impl Error {
    pub fn new(kind: Kind) -> Error {
        Error { kind }
    }

    pub fn new_reqwest_error_with_kind(external: ::reqwest::Error, kind: Kind) -> Error {
        let external_kind = ExternalKind::Reqwest(external, Some(Box::new(kind)));
        Error {
            kind: Kind::ExternalLib(external_kind),
        }
    }

    fn new_kind_from_code(code: StatusCode, err: CdfApiError) -> Kind {
        match code {
            StatusCode::BAD_REQUEST => Kind::BadRequest(err),
            StatusCode::UNAUTHORIZED => Kind::Unauthorized(err),
            StatusCode::FORBIDDEN => Kind::Forbidden(err),
            StatusCode::NOT_FOUND => Kind::NotFound(err),
            StatusCode::CONFLICT => Kind::Conflict(err),
            StatusCode::UNPROCESSABLE_ENTITY => Kind::UnprocessableEntity(err),
            _ => Kind::OtherApiError(err),
        }
    }

    pub fn new_from_cdf(
        code: StatusCode,
        err: ApiErrorWrapper,
        request_id: Option<String>,
    ) -> Error {
        let cdf_err = CdfApiError::new(err.error, request_id);
        let kind = Self::new_kind_from_code(code, cdf_err);
        Error { kind }
    }
    pub fn new_without_json(code: StatusCode, err: String, request_id: Option<String>) -> Error {
        let err = CdfApiError {
            code: code.to_owned().as_u16().into(),
            message: err,
            request_id,
            ..Default::default()
        };
        let kind = Self::new_kind_from_code(code, err);
        Error { kind }
    }
}

/// A `Result` alias where the `Err` case is `cognite::Error`.
pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Debug, Error)]
pub enum Kind {
    #[error("Bad request (400): {0}")]
    BadRequest(CdfApiError),
    #[error("Unauthorized (401): {0}")]
    Unauthorized(CdfApiError),
    #[error("Forbidden (403): {0}")]
    Forbidden(CdfApiError),
    #[error("Not Found (404): {0}")]
    NotFound(CdfApiError),
    #[error("Conflict (409): {0}")]
    Conflict(CdfApiError),
    #[error("Unprocessable Entity (422): {0}")]
    UnprocessableEntity(CdfApiError),
    #[error("Other Api Error: {0}")]
    OtherApiError(CdfApiError),
    #[error("Other HTTP error: {0}")]
    Http(String),
    #[error("Environment Variable Missing: {0}")]
    EnvironmentVariableMissing(String),
    #[error("{0}")]
    ExternalLib(ExternalKind),
    #[error("Error from authenticator: {0}")]
    Authenticator(AuthenticatorError),
    #[error("Invalid header value: {0}")]
    InvalidHeader(InvalidHeaderValue),
    #[error("Error accessing file: {0}")]
    IOError(std::io::Error),
    #[error("Error collecting stream: {0}")]
    StreamError(String),
}

#[derive(Debug, Error)]
pub enum ExternalKind {
    #[error("Unexpected request error: {0}")]
    Reqwest(::reqwest::Error, Option<Box<Kind>>),
    #[error("Unexpected JSON error: {0}")]
    SerdeJson(::serde_json::Error, Option<Box<Kind>>),
    #[error("Unexpected protobuf error: {0}")]
    Prost(::prost::DecodeError, Option<Box<Kind>>),
}

impl From<::reqwest::Error> for Kind {
    fn from(err: ::reqwest::Error) -> Kind {
        Kind::ExternalLib(ExternalKind::Reqwest(err, None))
    }
}
impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::new(Kind::from(err))
    }
}

impl From<::serde_json::Error> for Kind {
    fn from(err: ::serde_json::Error) -> Kind {
        Kind::ExternalLib(ExternalKind::SerdeJson(err, None))
    }
}
impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::new(Kind::from(err))
    }
}

impl From<::prost::DecodeError> for Kind {
    fn from(err: ::prost::DecodeError) -> Kind {
        Kind::ExternalLib(ExternalKind::Prost(err, None))
    }
}
impl From<::prost::DecodeError> for Error {
    fn from(err: ::prost::DecodeError) -> Error {
        Error::new(Kind::from(err))
    }
}

impl From<AuthenticatorError> for Kind {
    fn from(err: AuthenticatorError) -> Kind {
        Kind::Authenticator(err)
    }
}
impl From<AuthenticatorError> for Error {
    fn from(err: AuthenticatorError) -> Error {
        Error::new(Kind::from(err))
    }
}

impl From<InvalidHeaderValue> for Kind {
    fn from(err: InvalidHeaderValue) -> Self {
        Kind::InvalidHeader(err)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Error {
        Error::new(Kind::from(err))
    }
}

impl From<std::io::Error> for Kind {
    fn from(err: std::io::Error) -> Self {
        Kind::IOError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::new(Kind::from(err))
    }
}
