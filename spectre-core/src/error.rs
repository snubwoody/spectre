use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

// FIXME parse custom errors
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to send web socket message")]
    FailedToSendMessage,
    #[error("Failed to create new page")]
    FailedToCreatePage,
    #[error("Failed to parse CDP response: {0}")]
    InvalidResponse(String),
    #[error("{0}")]
    NavigationError(String),

    #[error("CDP Error: {message}")]
    CDPError { code: i32, message: String },

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
pub struct CDPError {
    id: i32,
    error: CDPErrorBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDPErrorBody {
    code: i32,
    message: String,
}
