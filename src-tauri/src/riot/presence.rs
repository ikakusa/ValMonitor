use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use std::sync::atomic::Ordering;

use crate::{
    errors::{AppError, AppResult},
    models::presence::PresenceEntry,
};

impl super::ValorantApi {
    pub async fn get_my_presence(&self) -> AppResult<PresenceEntry> {
        while !self.auth_initialized.load(Ordering::Acquire) {
            self.auth_initialized_notify.notified().await;
        }

        let puuid = self.puuid()?;
        self.get_presence()
            .await?
            .presences
            .into_iter()
            .find(|presence| presence.puuid == puuid)
            .ok_or_else(|| AppError::InvalidResponse("my presence was not found".to_string()))
    }

    pub async fn get_private_presence(&self) -> AppResult<Value> {
        let private =
            self.get_my_presence().await?.private.ok_or_else(|| {
                AppError::InvalidResponse("private presence is missing".to_string())
            })?;

        // Riot Local API の private presence は base64 JSON として返る。
        // 復号処理をここに閉じ込めることで、command / frontend にエンコード仕様を漏らさない。
        let decoded = String::from_utf8(general_purpose::STANDARD.decode(private)?)?;
        serde_json::from_str(&decoded).map_err(|error| AppError::InvalidResponse(error.to_string()))
    }

    pub async fn get_gamestate(&self) -> AppResult<String> {
        match self.get_private_presence().await {
            Ok(json) => json["matchPresenceData"]["sessionLoopState"]
                .as_str()
                .map(String::from)
                .ok_or_else(|| {
                    AppError::InvalidResponse("sessionLoopState is missing".to_string())
                }),
            Err(_) => Ok(String::from("IDLE")),
        }
    }
}
