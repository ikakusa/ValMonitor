use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Riot Client の lockfile が見つかりません")]
    LockfileNotFound,

    #[error("Riot session is not ready yet: {0}")]
    RiotSessionNotReady(String),

    #[error("Riot Storefront is not available right now: {0}")]
    StorefrontUnavailable(String),

    #[error("Riot API のレスポンス形式が想定外です: {0}")]
    InvalidResponse(String),

    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("IO failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("environment variable failed: {0}")]
    Env(#[from] std::env::VarError),

    #[error("base64 decode failed: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("utf8 decode failed: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("header value is invalid: {0}")]
    Header(#[from] reqwest::header::InvalidHeaderValue),

    #[error("shared state lock was poisoned: {0}")]
    StatePoisoned(&'static str),

    #[error("Henrik API key が設定されていません。%APPDATA%\\ValMonitor\\api.txt に API key を保存してください")]
    HenrikApiKeyMissing,

    #[error("Henrik API request failed with status {status}: {body}")]
    HenrikRequestFailed { status: u16, body: String },
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ErrorResponse {
            kind: &'static str,
            message: String,
        }

        // UI 側で再試行・警告表示・空状態を出し分けられるように、
        // 表示文だけでなく機械判定できる kind を必ず返す。
        let kind = match self {
            AppError::LockfileNotFound => "lockfileNotFound",
            AppError::RiotSessionNotReady(_) => "riotSessionNotReady",
            AppError::StorefrontUnavailable(_) => "storefrontUnavailable",
            AppError::InvalidResponse(_) => "invalidResponse",
            AppError::RequestFailed(_) => "requestFailed",
            AppError::Io(_) => "io",
            AppError::Env(_) => "environment",
            AppError::Base64(_) => "base64",
            AppError::Utf8(_) => "utf8",
            AppError::Header(_) => "header",
            AppError::StatePoisoned(_) => "statePoisoned",
            AppError::HenrikApiKeyMissing => "henrikApiKeyMissing",
            AppError::HenrikRequestFailed { .. } => "henrikRequestFailed",
        };

        ErrorResponse {
            kind,
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}
