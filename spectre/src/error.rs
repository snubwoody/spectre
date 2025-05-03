use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to send web socket message")]
    FailedToSendMessage,
    #[error("Failed to create new page")]
    FailedToCreatePage,
    #[error("Failed to get home directory")]
    FailedToGetHomeDir,
    #[error("Could not locate chrome binary")]
    MissingBinary,
    #[error("Failed to parse CDP response: {0}")]
    InvalidResponse(String),
    #[error("{0}")]
    NavigationError(String),
    #[error("{0}")]
    PageError(String),
    /// An error that occured while evaluating javascript
    /// in browser
    #[error("Uncaught expection: {description}")]
    RuntimeError {
        line_number: i32,
        column_number: i32,
        value: Option<Value>,
        description: String,
    },
    #[error("CDP Error: {message}")]
    CDPError { code: i32, message: String },

    // Third party errors
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),
}

impl From<CDPError> for Error {
    fn from(value: CDPError) -> Self {
        Error::CDPError {
            code: value.error.code,
            message: value.error.message,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CDPError {
    id: i32,
    session_id: Option<String>,
    error: CDPErrorBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDPErrorBody {
    code: i32,
    message: String,
}
