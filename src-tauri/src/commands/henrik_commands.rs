use crate::riot_api::{clients::henrik_models::Account, core::VALORANT_API};

#[tauri::command]
pub async fn get_player_by_id(id: String) -> Result<Account, String> {
    VALORANT_API.henrik_client.get_account_by_id(&id).await.map_err(|e| e.to_string())
}