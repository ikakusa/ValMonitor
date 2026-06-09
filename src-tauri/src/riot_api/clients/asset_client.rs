use reqwest::Client;
use std::sync::OnceLock;

use super::asset_models::{PlayerCard, RiotVersion};
use crate::riot_api::response::{RequestError, RiotResponse};

pub struct AssetClient;

impl AssetClient {
    fn http_client() -> &'static Client {
        static CLIENT: OnceLock<Client> = OnceLock::new();
        CLIENT.get_or_init(|| Client::new())
    }

    pub async fn send_request(path: &str) -> Result<RiotResponse, RequestError> {
        let response = Self::http_client()
            .get(format!("https://valorant-api.com{path}"))
            .send()
            .await?;
        let status = response.status();

        if !status.is_success() {
            return Err(RequestError::FailedWithStatus(status));
        }

        let bytes = response.bytes().await?;
        Ok(RiotResponse {
            bytes: bytes,
            status: status,
        })
    }

    pub async fn get_player_card(uuid: &str) -> Result<PlayerCard, RequestError> {
        let res = Self::send_request(format!("/v1/playercards/{}", uuid).as_str())
            .await?
            .get_json::<PlayerCard>()?;
        Ok(res)
    }

    pub async fn get_version() -> Result<RiotVersion, RequestError> {
        let res = Self::send_request("/v1/version")
            .await?
            .get_json::<RiotVersion>()?;
        Ok(res)
    }
}
