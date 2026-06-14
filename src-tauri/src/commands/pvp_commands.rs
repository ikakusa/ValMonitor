use crate::{
    riot_api::clients::pvp_models::{PregameMatch, MMR},
    VALORANT_API,
};

#[tauri::command]
pub async fn get_player_mmr(uid: String) -> Result<MMR, String> {
    VALORANT_API
        .pvp_client
        .get_mmr(uid.as_str(), false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_current_pregame() -> Result<PregameMatch, String> {
    VALORANT_API
        .pvp_client
        .get_current_pregame(false)
        .await
        .map_err(|e| e.to_string())
}