use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{sync::atomic::Ordering, time::Instant};

use super::MutexExt;
use crate::{
    errors::{AppError, AppResult},
    http_log,
    models::{
        auth::EntitlementsResponse,
        lockfile::LockfileData,
        presence::{ExternalSession, ExternalSessionsResponse, PresenceResponse},
    },
};

impl super::ValorantApi {
    pub async fn get_presence(&self) -> AppResult<PresenceResponse> {
        self.get_local_request("/chat/v4/presences").await
    }

    pub async fn get_region(&self) -> AppResult<String> {
        if let Some(region) = self.region.lock_app("region")?.clone() {
            return Ok(region);
        }

        let sessions = self
            .get_local_request::<ExternalSessionsResponse>("/product-session/v1/external-sessions")
            .await?;

        let session_json = sessions
            .into_iter()
            .filter(|(key, _)| key != "host_app")
            .map(|(_, value)| value)
            .find(has_region_argument)
            .ok_or_else(|| {
                AppError::RiotSessionNotReady(
                    "Valorant external session with region was not found".to_string(),
                )
            })?;
        let session = serde_json::from_value::<ExternalSession>(session_json)
            .map_err(|error| AppError::InvalidResponse(error.to_string()))?;

        let region = extract_region(session)?;
        *self.region.lock_app("region")? = Some(region.clone());
        tracing::debug!(region, "Valorant region detected");

        Ok(region)
    }

    pub async fn get_entitlements_token(&self) -> AppResult<EntitlementsResponse> {
        self.get_local_request("/entitlements/v1/token").await
    }

    pub(super) fn set_local_session(&self, lockfile: LockfileData) -> AppResult<()> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(lockfile.authorization_header().as_str())?,
        );

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(default_headers)
            .build()?;

        tracing::debug!(
            name = lockfile.name,
            pid = lockfile.pid,
            protocol = lockfile.protocol,
            port = lockfile.port,
            "Riot Local API session updated"
        );

        *self.raw_lockfile.lock_app("raw_lockfile")? = lockfile.raw.clone();
        *self.local_session.lock_app("local_session")? =
            Some(super::LocalSession { lockfile, client });
        *self.region.lock_app("region")? = None;

        // Local API が使えるようになった瞬間に待機中の command / service を起こす。
        // lockfile 再生成時も同じ notify を使うことで、呼び出し側は初回と更新を区別せず待てる。
        self.local_initialized.store(true, Ordering::Release);
        self.local_initialized_notify.notify_waiters();

        Ok(())
    }

    pub(super) async fn get_local_request<T>(&self, path: &str) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        while !self.local_initialized.load(Ordering::Acquire) {
            self.local_initialized_notify.notified().await;
        }

        let session = self
            .local_session
            .lock_app("local_session")?
            .clone()
            .ok_or_else(|| AppError::InvalidResponse("local session is missing".to_string()))?;

        let started = Instant::now();
        let response = match session
            .client
            .get(format!(
                "https://127.0.0.1:{}{}",
                session.lockfile.port, path
            ))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                http_log::transport_error("riot-local", "GET", path, started.elapsed(), &error);
                return Err(error.into());
            }
        };
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            http_log::response_error("riot-local", "GET", path, status.as_u16(), started.elapsed(), &body);
            return Err(AppError::RiotSessionNotReady(format!(
                "Riot Local API returned status {} for {path}",
                status.as_u16()
            )));
        }

        http_log::response("riot-local", "GET", path, status.as_u16(), started.elapsed(), body.len());

        if body.trim().is_empty() {
            return Err(AppError::RiotSessionNotReady(format!(
                "Riot Local API returned an empty response for {path}"
            )));
        }

        let response = serde_json::from_str::<T>(&body).map_err(|error| {
            http_log::parse_error("riot-local", "GET", path, started.elapsed(), &error.to_string(), &body);
            AppError::InvalidResponse(format!(
                "failed to parse {path}: {error}; body starts with: {}",
                body.chars().take(120).collect::<String>()
            ))
        })?;

        Ok(response)
    }
}

fn has_region_argument(value: &Value) -> bool {
    value["launchConfiguration"]["arguments"]
        .as_array()
        .is_some_and(|arguments| {
            arguments.iter().filter_map(Value::as_str).any(|arg| {
                arg.starts_with("-ares-deployment=") || arg.starts_with("ares-deployment=")
            })
        })
}

fn extract_region(session: ExternalSession) -> AppResult<String> {
    let arg = session
        .launch_configuration
        .arguments
        .iter()
        .find(|arg| arg.starts_with("-ares-deployment=") || arg.starts_with("ares-deployment="))
        .or_else(|| session.launch_configuration.arguments.get(4))
        .ok_or_else(|| AppError::InvalidResponse("region argument not found".to_string()))?;

    arg.split('=')
        .nth(1)
        .map(ToString::to_string)
        .ok_or_else(|| AppError::InvalidResponse("invalid region format".to_string()))
}
