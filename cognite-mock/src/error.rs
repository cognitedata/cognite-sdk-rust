use thiserror::Error;

pub type Result<T> = std::result::Result<T, MockError>;

#[derive(Error, Debug, Clone)]
pub enum MockError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}
