use crate::riot_api::{
    clients::local_models::{Presence, PresenceData, PrivatePresence},
    core::VALORANT_API,
};
#[tauri::command]
pub async fn get_region() -> String {
    VALORANT_API.shared.get_user().clone().region
}

#[tauri::command]
pub async fn get_puuid() -> String {
    VALORANT_API.shared.get_user().clone().puuid
}

#[tauri::command]
pub async fn get_full_username() -> String {
    let user = { VALORANT_API.shared.get_user().clone() };
    format!("{}#{}", user.name, user.tag)
}

#[tauri::command]
pub async fn is_api_initialized() -> bool {
    VALORANT_API.is_initialized()
}

#[tauri::command]
pub async fn get_gamestate() -> Result<String, String> {
    VALORANT_API
        .local_client
        .get_gamestate(false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_private_presence() -> Result<PrivatePresence, String> {
    VALORANT_API
        .local_client
        .get_private_presence(false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_my_presence() -> Result<PresenceData, String> {
    VALORANT_API
        .local_client
        .get_my_presence(false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_presence() -> Result<Presence, String> {
    VALORANT_API
        .local_client
        .get_presences(false)
        .await
        .map_err(|e| e.to_string())
}
