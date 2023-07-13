use thiserror::Error;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("[Error]: {0}")]
    StringError(String),
}