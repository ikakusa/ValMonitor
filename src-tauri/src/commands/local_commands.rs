use std::str::FromStr;

use crate::riot_api::core::VALORANT_API;
use serde_json::Value;

#[tauri::command]
pub async fn get_region() -> Result<String, String> {
    Ok("".into())
    // VALORANT_API.get_region().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_puuid() -> String {
    VALORANT_API.shared.get_user().clone().puuid
}

#[tauri::command]
pub async fn get_full_username() -> String {
    let user = { VALORANT_API.shared.get_user().clone() };
    format!(
        "{}#{}",
        user.name,
        user.tag
    )
}

#[tauri::command]
pub async fn is_api_initialized() -> bool {
    VALORANT_API.is_initialized()
}

#[tauri::command]
pub async fn get_gamestate() -> Result<String, String> {
    Ok("".into())
    // VALORANT_API
    //     .get_gamestate()
    //     .await
    //     .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_private_presence() -> Result<Value, String> {
    Ok(Value::from_str("{}").unwrap())

    // VALORANT_API
    //     .get_private_presence()
    //     .await
    //     .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_my_presence() -> Result<Value, String> {
    Ok(Value::from_str("{}").unwrap())
    // VALORANT_API
    //     .get_my_presence()
    //     .await
    //     .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_presence() -> Result<Value, String> {
    Ok(Value::from_str("{}").unwrap())
    // VALORANT_API.get_presence().await.map_err(|e| e.to_string())
}