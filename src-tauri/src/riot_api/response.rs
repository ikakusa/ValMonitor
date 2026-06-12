use bytes::Bytes;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("request failed with status code: {0}")]
    FailedWithStatus(reqwest::StatusCode),

    #[error("json parse failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("An error has occurred: {0}")]
    Unknown(String),
}

pub struct RiotResponse {
    pub bytes: Bytes,
    pub status: reqwest::StatusCode,
}

impl RiotResponse {
    pub fn get_json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.bytes)
    }

    pub fn get_text(&self) -> Result<&str, std::str::Utf8Error> {
        Ok(std::str::from_utf8(&self.bytes))?
    }
}
