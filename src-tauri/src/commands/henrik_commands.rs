use crate::riot_api::{
    clients::henrik_models::{henrik_matchlist::MatchList, Account, PlayerStats},
    core::VALORANT_API,
};

#[tauri::command]
pub async fn get_player_by_id(id: String) -> Result<Account, String> {
    VALORANT_API
        .henrik_client
        .get_account_by_id(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_matchlist_by_puuid(id: String, q: Option<String>) -> Result<MatchList, String> {
    VALORANT_API
        .henrik_client
        .get_matchlist_by_puuid(&id, q)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_stats_by_puuid(id: String) -> Result<PlayerStats, String> {
    VALORANT_API
        .henrik_client
        .get_player_stats_by_id(&id)
        .await
        .map_err(|e| e.to_string())
}
