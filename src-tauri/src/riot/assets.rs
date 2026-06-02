use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Instant;

use crate::{
    errors::{AppError, AppResult},
    http_log,
    models::assets::{PlayerCardData, ValorantApiDataResponse, ValorantVersionData},
};

static VAL_ASSET_CLIENT: Lazy<Result<Client, reqwest::Error>> =
    Lazy::new(|| Client::builder().user_agent("ValMonitor").build());

fn val_asset_client() -> AppResult<&'static Client> {
    // asset API は Riot Local API と違い通常の HTTPS 通信なので、専用 client として分ける。
    // 起動時 panic にせず AppError に落とすことで、UI 側で assets だけ失敗した状態を扱える。
    VAL_ASSET_CLIENT.as_ref().map_err(|error| {
        AppError::InvalidResponse(format!("failed to build asset client: {error}"))
    })
}

impl super::ValorantApi {
    pub async fn get_playercard_by_id(&self, id: String) -> AppResult<PlayerCardData> {
        let path = format!("/v1/playercards/{id}");
        let started = Instant::now();
        let response = match val_asset_client()?
            .get(format!("https://valorant-api.com{path}"))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                http_log::transport_error("valorant-assets", "GET", &path, started.elapsed(), &error);
                return Err(error.into());
            }
        };
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            http_log::response_error("valorant-assets", "GET", &path, status.as_u16(), started.elapsed(), &body);
            return Err(AppError::RiotSessionNotReady(format!(
                "Valorant asset API returned status {} for {path}",
                status.as_u16()
            )));
        }
        http_log::response("valorant-assets", "GET", &path, status.as_u16(), started.elapsed(), body.len());
        let response = serde_json::from_str::<ValorantApiDataResponse<PlayerCardData>>(&body).map_err(|error| {
            http_log::parse_error("valorant-assets", "GET", &path, started.elapsed(), &error.to_string(), &body);
            AppError::InvalidResponse(format!("failed to parse asset {path}: {error}"))
        })?;

        Ok(response.data)
    }

    pub(super) async fn get_riot_client_version(&self) -> AppResult<String> {
        let path = "/v1/version";
        let started = Instant::now();
        let response = match val_asset_client()?
            .get(format!("https://valorant-api.com{path}"))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                http_log::transport_error("valorant-assets", "GET", path, started.elapsed(), &error);
                return Err(error.into());
            }
        };
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            http_log::response_error("valorant-assets", "GET", path, status.as_u16(), started.elapsed(), &body);
            return Err(AppError::RiotSessionNotReady(format!(
                "Valorant asset API returned status {} for {path}",
                status.as_u16()
            )));
        }
        http_log::response("valorant-assets", "GET", path, status.as_u16(), started.elapsed(), body.len());
        let response = serde_json::from_str::<ValorantApiDataResponse<ValorantVersionData>>(&body).map_err(|error| {
            http_log::parse_error("valorant-assets", "GET", path, started.elapsed(), &error.to_string(), &body);
            AppError::InvalidResponse(format!("failed to parse asset {path}: {error}"))
        })?;

        Ok(response.data.riot_client_version)
    }
}
