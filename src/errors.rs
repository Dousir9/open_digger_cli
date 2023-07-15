use reqwest;
use serde_json;
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("[Error]: {0}")]
    String(String),
    #[error("[Requset Error]: {0}")]
    Request(#[from] reqwest::Error),
    #[error("[Seld Error]: {0}")]
    Sled(#[from] serde_json::Error),
    #[error("[I/O Error]: {0}")]
    Io(#[from] io::Error),
}
