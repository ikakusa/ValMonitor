use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::{
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

use super::MutexExt;
use crate::{
    errors::{AppError, AppResult},
    http_log,
    models::pvp::{PlayerLoadoutResponse, PvpMmrResponse, StorefrontResponse},
};
use serde_json::{json, Value};

const RIOT_CLIENT_PLATFORM: &str = "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9";

impl super::ValorantApi {
    pub async fn get_mmr(&self, puuid: &str) -> AppResult<PvpMmrResponse> {
        self.get_pvp_request(format!("/mmr/v1/players/{puuid}").as_str())
            .await
    }

    pub async fn get_player_loadout(&self, puuid: &str) -> AppResult<PlayerLoadoutResponse> {
        self.get_pvp_request(format!("/personalization/v2/players/{puuid}/playerloadout").as_str())
            .await
    }

    pub async fn get_storefront(&self, puuid: &str) -> AppResult<StorefrontResponse> {
        let v2_path = format!("/store/v2/storefront/{puuid}");
        let v3_path = format!("/store/v3/storefront/{puuid}");
        let storefront = match self.get_pvp_request::<StorefrontResponse>(v2_path.as_str()).await {
            Ok(storefront) => storefront,
            Err(AppError::RiotSessionNotReady(message)) if message.contains("status 404") => {
                log::debug!("HTTP riot-pvp GET {v2_path} -> storefront_v3_fallback");
                self.get_pvp_request::<StorefrontResponse>(v3_path.as_str())
                    .await
                    .map_err(|error| AppError::StorefrontUnavailable(error.to_string()))?
            }
            Err(error) => return Err(AppError::StorefrontUnavailable(error.to_string())),
        };
        let single_item_offers = storefront
            .skins_panel_layout
            .as_ref()
            .map(|layout| layout.single_item_offers.len())
            .unwrap_or_default();
        let single_item_store_offers = storefront
            .skins_panel_layout
            .as_ref()
            .map(|layout| layout.single_item_store_offers.len())
            .unwrap_or_default();

        log::debug!(
            "Storefront parsed: single_item_offers={single_item_offers}, single_item_store_offers={single_item_store_offers}"
        );

        Ok(storefront)
    }

    pub async fn get_current_match(&self, puuid: &str) -> AppResult<Value> {
        if let Ok(player) = self
            .get_glz_request_once::<Value>(format!("/core-game/v1/players/{puuid}").as_str())
            .await
        {
            let match_id = extract_match_id(&player)?;
            let game = self
                .get_glz_request_once::<Value>(format!("/core-game/v1/matches/{match_id}").as_str())
                .await?;
            return Ok(json!({ "mode": "core-game", "player": player, "match": game }));
        }

        let player = self
            .get_glz_request_once::<Value>(format!("/pregame/v1/players/{puuid}").as_str())
            .await?;
        let match_id = extract_match_id(&player)?;
        let game = self
            .get_glz_request_once::<Value>(format!("/pregame/v1/matches/{match_id}").as_str())
            .await?;
        Ok(json!({ "mode": "pregame", "player": player, "match": game }))
    }

    pub async fn get_pvp_request<T>(&self, path: &str) -> AppResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match self.get_pvp_request_once(path).await {
            Ok(response) => Ok(response),
            Err(first_error) => {
                // access token 更新直後や Riot ClientVersion 変更時だけ 1 回再構築する。
                // 404/403 でも無限 retry すると Live View や Store 表示が固まるため、2 回目は UI へ error として返す。
                log::debug!("HTTP riot-pvp GET {path} -> retry_once reason=\"{first_error}\"");
                self.build_pvp_client().await?;
                tokio::time::sleep(Duration::from_millis(250)).await;
                self.get_pvp_request_once(path).await
            }
        }
    }

    async fn get_pvp_request_once<T>(&self, path: &str) -> AppResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        while !self.pvp_initialized.load(Ordering::Acquire) {
            self.pvp_initialized_notify.notified().await;
        }

        let client = self
            .pvp_client
            .lock_app("pvp_client")?
            .clone()
            .ok_or_else(|| AppError::InvalidResponse("PVP client is missing".to_string()))?;
        let region = self.get_region().await?;

        let started = Instant::now();
        let response = match client
            .get(format!("https://pd.{region}.a.pvp.net{path}"))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                http_log::transport_error("riot-pvp", "GET", path, started.elapsed(), &error);
                return Err(error.into());
            }
        };
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            http_log::response_error("riot-pvp", "GET", path, status.as_u16(), started.elapsed(), &body);
            return Err(AppError::RiotSessionNotReady(format!(
                "PVP API returned status {} for {path}: {}",
                status.as_u16(),
                body.chars().take(120).collect::<String>()
            )));
        }

        if body.trim().is_empty() {
            return Err(AppError::RiotSessionNotReady(format!(
                "PVP API returned an empty response for {path}"
            )));
        }

        http_log::response("riot-pvp", "GET", path, status.as_u16(), started.elapsed(), body.len());
        serde_json::from_str::<T>(&body).map_err(|error| {
            http_log::parse_error("riot-pvp", "GET", path, started.elapsed(), &error.to_string(), &body);
            AppError::InvalidResponse(format!(
                "failed to parse PVP {path}: {error}; body starts with: {}",
                body.chars().take(120).collect::<String>()
            ))
        })
    }

    async fn get_glz_request_once<T>(&self, path: &str) -> AppResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        while !self.pvp_initialized.load(Ordering::Acquire) {
            self.pvp_initialized_notify.notified().await;
        }

        let client = self
            .pvp_client
            .lock_app("pvp_client")?
            .clone()
            .ok_or_else(|| AppError::InvalidResponse("PVP client is missing".to_string()))?;
        let region = self.get_region().await?;

        let started = Instant::now();
        let response = match client
            .get(format!("https://glz-{region}-1.{region}.a.pvp.net{path}"))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                http_log::transport_error("riot-glz", "GET", path, started.elapsed(), &error);
                return Err(error.into());
            }
        };
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            http_log::response_error("riot-glz", "GET", path, status.as_u16(), started.elapsed(), &body);
            return Err(AppError::RiotSessionNotReady(format!(
                "GLZ API returned status {} for {path}: {}",
                status.as_u16(),
                body.chars().take(120).collect::<String>()
            )));
        }

        if body.trim().is_empty() {
            return Err(AppError::RiotSessionNotReady(format!(
                "GLZ API returned an empty response for {path}"
            )));
        }

        http_log::response("riot-glz", "GET", path, status.as_u16(), started.elapsed(), body.len());
        serde_json::from_str::<T>(&body).map_err(|error| {
            http_log::parse_error("riot-glz", "GET", path, started.elapsed(), &error.to_string(), &body);
            AppError::InvalidResponse(format!(
                "failed to parse GLZ {path}: {error}; body starts with: {}",
                body.chars().take(120).collect::<String>()
            ))
        })
    }

    pub(super) async fn build_pvp_client(&self) -> AppResult<()> {
        let snapshot = self.auth_snapshot.lock_app("auth_snapshot")?.clone();
        let client_version = self.get_riot_client_version().await?;

        let mut pvp_headers = HeaderMap::new();
        pvp_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", snapshot.access_token).as_str())?,
        );
        pvp_headers.insert(
            "X-Riot-Entitlements-JWT",
            HeaderValue::from_str(snapshot.entitlements_token.as_str())?,
        );
        pvp_headers.insert(
            "X-Riot-ClientPlatform",
            HeaderValue::from_str(RIOT_CLIENT_PLATFORM)?,
        );
        pvp_headers.insert(
            "X-Riot-ClientVersion",
            HeaderValue::from_str(client_version.as_str())?,
        );

        *self.pvp_client.lock_app("pvp_client")? = Some(
            reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .default_headers(pvp_headers)
                .build()?,
        );

        // PVP API は access token / entitlements / client version の全てが揃って初めて使える。
        // そのため local/auth とは別の initialized flag として扱う。
        self.pvp_initialized.store(true, Ordering::Release);
        self.pvp_initialized_notify.notify_waiters();

        Ok(())
    }
}

fn extract_match_id(value: &Value) -> AppResult<&str> {
    value["MatchID"]
        .as_str()
        .ok_or_else(|| AppError::RiotSessionNotReady("current match id is missing".to_string()))
}
