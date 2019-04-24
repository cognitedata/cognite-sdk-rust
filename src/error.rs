use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorWrapper {
  pub error : ApiErrorMessage
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorMessage {
  pub code : u32,
  pub message : String,
}

pub struct Error {
  kind : Kind
}

impl Error {
  pub fn new(kind : Kind) -> Error {
    Error {
      kind : kind
    }
  }
}

/// A `Result` alias where the `Err` case is `cognite::Error`.
pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .finish()
    }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.kind {
      Kind::Reqwest(ref e) => fmt::Display::fmt(e, f),
      Kind::Unauthorized(ref e) => f.write_str(e),
      Kind::Forbidden(ref e) => f.write_str(e),
      Kind::Http(ref e) => f.write_str(e)
    }
  }
}

impl StdError for Error {
  fn description(&self) -> &str {
    match self.kind {
      Kind::Reqwest(ref e) => e.description(),
      Kind::Unauthorized(ref e) => e,
      Kind::Forbidden(ref e) => e,
      Kind::Http(ref e) => e,
    }
  }

  #[allow(deprecated)]
  fn cause(&self) -> Option<&StdError> {
    None
  }
}

#[derive(Debug)]
pub enum Kind {
  Unauthorized(String),
  Forbidden(String),
  Http(String),
  Reqwest(::reqwest::Error)
}

impl From<::reqwest::Error> for Kind {
  #[inline]
  fn from(err: ::reqwest::Error) -> Kind {
      Kind::Reqwest(err)
  }
}
impl From<::reqwest::Error> for Error {
  fn from(err: ::reqwest::Error) -> Error {
      Error::new(Kind::from(err))
  }
}