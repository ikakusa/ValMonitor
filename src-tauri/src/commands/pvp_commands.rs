use crate::{VALORANT_API, riot_api::clients::pvp_models::MMR};

#[tauri::command]
pub async fn get_player_mmr(uid: String) -> Result<MMR, String> {
    VALORANT_API.pvp_client.get_mmr(uid.as_str(), false).await.map_err(|e| e.to_string())
}