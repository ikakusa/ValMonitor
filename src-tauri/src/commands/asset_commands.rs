use crate::riot_api::clients::{
    asset_client::AssetClient,
    asset_models::{PlayerCard, RiotVersion},
};

#[tauri::command]
pub async fn get_playercard_by_id(id: String) -> Result<PlayerCard, String> {
    AssetClient::get_player_card(id.as_str())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_riot_version() -> Result<RiotVersion, String> {
    AssetClient::get_version().await.map_err(|e| e.to_string())
}
