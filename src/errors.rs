use thiserror::Error;
use reqwest::Error as RequestError;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("[Error]: {0}")]
    StringError(String),
    #[error("[Error]: {0}")]
    RequestError(String),
}

impl From<RequestError> for CliError {
    fn from(value: RequestError) -> Self {
        Self::RequestError(value.to_string())
    }
}