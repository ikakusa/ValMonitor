use serde_json::Value;

use crate::{
    errors::{AppError, AppResult},
    models::{
        assets::PlayerCardData,
        presence::PresenceEntry,
        pvp::{PlayerLoadoutResponse, PvpMmrResponse, StorefrontResponse},
    },
    state::AppState,
};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn get_player_mmr(
    state: tauri::State<'_, AppState>,
    uid: String,
) -> AppResult<PvpMmrResponse> {
    state.riot.get_mmr(uid.as_str()).await
}

#[tauri::command]
pub async fn get_player_loadout(
    state: tauri::State<'_, AppState>,
    uid: String,
) -> AppResult<PlayerLoadoutResponse> {
    state.riot.get_player_loadout(uid.as_str()).await
}

#[tauri::command]
pub async fn get_storefront(
    state: tauri::State<'_, AppState>,
    uid: String,
) -> AppResult<StorefrontResponse> {
    state.riot.get_storefront(uid.as_str()).await
}

#[tauri::command]
pub async fn get_current_match(state: tauri::State<'_, AppState>, uid: String) -> AppResult<Value> {
    state.riot.get_current_match(uid.as_str()).await
}

#[tauri::command]
pub async fn get_presence(state: tauri::State<'_, AppState>) -> AppResult<String> {
    // 既存 frontend は JSON 文字列として受け取る前提が残っているため、
    // command 名を維持しつつ内部だけ型付き DTO に移行する。
    serde_json::to_string(&state.riot.get_presence().await?)
        .map_err(|error| AppError::InvalidResponse(error.to_string()))
}

#[tauri::command]
pub async fn get_all_presences(state: tauri::State<'_, AppState>) -> AppResult<Vec<PresenceEntry>> {
    Ok(state.riot.get_presence().await?.presences)
}

#[tauri::command]
pub async fn get_auth_userinfo(state: tauri::State<'_, AppState>) -> AppResult<String> {
    // ここも frontend 移行が終わるまでは文字列返却を維持する。
    // Rust 側では UserInfoResponse として parse しているので、API 境界の型安全性は上がっている。
    serde_json::to_string(&state.riot.get_userinfo().await?)
        .map_err(|error| AppError::InvalidResponse(error.to_string()))
}

#[tauri::command]
pub async fn get_my_presence(state: tauri::State<'_, AppState>) -> AppResult<PresenceEntry> {
    state.riot.get_my_presence().await
}

#[tauri::command]
pub async fn get_private_presence(state: tauri::State<'_, AppState>) -> AppResult<Value> {
    state.riot.get_private_presence().await
}

#[tauri::command]
pub async fn get_gamestate(state: tauri::State<'_, AppState>) -> AppResult<String> {
    state.riot.get_gamestate().await
}

#[tauri::command]
pub async fn is_api_initialized(state: tauri::State<'_, AppState>) -> AppResult<bool> {
    Ok(state.riot.is_initialized())
}

#[tauri::command]
pub async fn get_full_username(state: tauri::State<'_, AppState>) -> AppResult<String> {
    state.riot.full_username()
}

#[tauri::command]
pub async fn get_puuid(state: tauri::State<'_, AppState>) -> AppResult<String> {
    state.riot.puuid()
}

#[tauri::command]
pub async fn get_playercard_by_id(
    state: tauri::State<'_, AppState>,
    id: String,
) -> AppResult<PlayerCardData> {
    state.riot.get_playercard_by_id(id).await
}

#[tauri::command]
pub async fn get_region(state: tauri::State<'_, AppState>) -> AppResult<String> {
    state.riot.get_region().await
}
