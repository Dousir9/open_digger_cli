use thiserror::Error;
use reqwest::Error as RequestError;
use serde_json::Error as JsonError;
use std::io::Error as IoError;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("[Error]: {0}")]
    StringError(String),
    #[error("[Error]: {0}")]
    RequestError(String),
    #[error("[Error]: {0}")]
    JsonError(String),
    #[error("[Error]: {0}")]
    IoError(String)
}

impl From<RequestError> for CliError {
    fn from(value: RequestError) -> Self {
        Self::RequestError(value.to_string())
    }
}

impl From<JsonError> for CliError {
    fn from(value: JsonError) -> Self {
        Self::RequestError(value.to_string())
    }
}

impl From<IoError> for CliError {
    fn from(value: IoError) -> Self {
        Self::IoError(value.to_string())
    }
}