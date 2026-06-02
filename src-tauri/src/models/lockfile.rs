use base64::{engine::general_purpose, Engine as _};

use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct LockfileData {
    pub name: String,
    pub pid: String,
    pub port: String,
    pub password: String,
    pub protocol: String,
    pub raw: String,
}

impl LockfileData {
    pub fn parse(raw: String) -> AppResult<Self> {
        let split = raw.split(':').collect::<Vec<_>>();
        if split.len() < 5 {
            return Err(AppError::InvalidResponse(
                "lockfile must contain name, pid, port, password and protocol".to_string(),
            ));
        }

        Ok(Self {
            name: split[0].to_string(),
            pid: split[1].to_string(),
            port: split[2].to_string(),
            password: split[3].to_string(),
            protocol: split[4].to_string(),
            raw,
        })
    }

    pub fn authorization_header(&self) -> String {
        format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("riot:{}", self.password))
        )
    }
}
