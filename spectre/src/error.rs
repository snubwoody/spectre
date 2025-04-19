use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T,Error>;

// FIXME parse custom errors
#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to send web socket message")]
	FailedToSendMessage,

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
