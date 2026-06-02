use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::de::DeserializeOwned;
use std::{sync::atomic::Ordering, time::Instant};

use super::MutexExt;
use crate::{
    errors::{AppError, AppResult},
    http_log,
    models::auth::UserInfoResponse,
};

impl super::ValorantApi {
    pub async fn get_userinfo(&self) -> AppResult<UserInfoResponse> {
        self.get_auth_request("/userinfo").await
    }

    pub(super) fn set_auth_session(&self, access_token: &str) -> AppResult<()> {
        let mut auth_headers = HeaderMap::new();
        auth_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {access_token}").as_str())?,
        );

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(auth_headers)
            .build()?;

        *self.auth_session.lock_app("auth_session")? = Some(super::AuthSession { client });

        // userinfo と presence 復号は Bearer token に依存するため、
        // token 更新が終わるまで caller を待機させる。
        self.auth_initialized.store(true, Ordering::Release);
        self.auth_initialized_notify.notify_waiters();

        Ok(())
    }

    pub(super) async fn get_auth_request<T>(&self, path: &str) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        while !self.auth_initialized.load(Ordering::Acquire) {
            self.auth_initialized_notify.notified().await;
        }

        let session = self
            .auth_session
            .lock_app("auth_session")?
            .clone()
            .ok_or_else(|| AppError::InvalidResponse("auth session is missing".to_string()))?;

        let started = Instant::now();
        let response = match session
            .client
            .get(format!("https://auth.riotgames.com{path}"))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                http_log::transport_error("riot-auth", "GET", path, started.elapsed(), &error);
                return Err(error.into());
            }
        };
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            http_log::response_error("riot-auth", "GET", path, status.as_u16(), started.elapsed(), &body);
            return Err(AppError::RiotSessionNotReady(format!(
                "Riot auth API returned status {} for {path}",
                status.as_u16()
            )));
        }

        http_log::response("riot-auth", "GET", path, status.as_u16(), started.elapsed(), body.len());

        if body.trim().is_empty() {
            return Err(AppError::RiotSessionNotReady(format!(
                "Riot auth API returned an empty response for {path}"
            )));
        }

        let response = serde_json::from_str::<T>(&body).map_err(|error| {
            http_log::parse_error("riot-auth", "GET", path, started.elapsed(), &error.to_string(), &body);
            AppError::InvalidResponse(format!(
                "failed to parse auth {path}: {error}; body starts with: {}",
                body.chars().take(120).collect::<String>()
            ))
        })?;

        Ok(response)
    }
}
