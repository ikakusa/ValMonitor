use serde_json::Value;

use crate::{
    errors::AppResult,
    henrik::{
        runtime_settings, save_api_key, HenrikAccountByNameRequest, HenrikAccountByPuuidRequest,
        HenrikContentRequest, HenrikCrosshairRequest, HenrikEsportsScheduleRequest,
        HenrikLeaderboardRequest, HenrikMatchRequest, HenrikMatchesByNameRequest,
        HenrikMatchesByPuuidRequest, HenrikMmrByNameRequest, HenrikMmrByPuuidRequest,
        HenrikMmrHistoryByNameRequest, HenrikMmrHistoryByPuuidRequest, HenrikRuntimeSettings,
        HenrikVlrEntityRequest,
    },
    state::AppState,
};

#[tauri::command]
pub async fn henrik_get_settings() -> AppResult<HenrikRuntimeSettings> {
    runtime_settings()
}

#[tauri::command]
pub async fn henrik_save_api_key(api_key: String) -> AppResult<HenrikRuntimeSettings> {
    save_api_key(api_key)
}

#[tauri::command]
pub async fn henrik_account_by_name(
    state: tauri::State<'_, AppState>,
    input: HenrikAccountByNameRequest,
) -> AppResult<Value> {
    state.henrik.account_by_name(input).await
}

#[tauri::command]
pub async fn henrik_account_by_puuid(
    state: tauri::State<'_, AppState>,
    input: HenrikAccountByPuuidRequest,
) -> AppResult<Value> {
    state.henrik.account_by_puuid(input).await
}

#[tauri::command]
pub async fn henrik_content(
    state: tauri::State<'_, AppState>,
    input: HenrikContentRequest,
) -> AppResult<Value> {
    state.henrik.content(input).await
}

#[tauri::command]
pub async fn henrik_crosshair(
    state: tauri::State<'_, AppState>,
    input: HenrikCrosshairRequest,
) -> AppResult<Value> {
    state.henrik.crosshair(input).await
}

#[tauri::command]
pub async fn henrik_esports_schedule(
    state: tauri::State<'_, AppState>,
    input: HenrikEsportsScheduleRequest,
) -> AppResult<Value> {
    state.henrik.esports_schedule(input).await
}

#[tauri::command]
pub async fn henrik_leaderboard(
    state: tauri::State<'_, AppState>,
    input: HenrikLeaderboardRequest,
) -> AppResult<Value> {
    state.henrik.leaderboard(input).await
}

#[tauri::command]
pub async fn henrik_matches_by_name(
    state: tauri::State<'_, AppState>,
    input: HenrikMatchesByNameRequest,
) -> AppResult<Value> {
    state.henrik.matches_by_name(input).await
}

#[tauri::command]
pub async fn henrik_matches_by_puuid(
    state: tauri::State<'_, AppState>,
    input: HenrikMatchesByPuuidRequest,
) -> AppResult<Value> {
    state.henrik.matches_by_puuid(input).await
}

#[tauri::command]
pub async fn henrik_match_by_id(
    state: tauri::State<'_, AppState>,
    input: HenrikMatchRequest,
) -> AppResult<Value> {
    state.henrik.match_by_id(input).await
}

#[tauri::command]
pub async fn henrik_mmr_by_name(
    state: tauri::State<'_, AppState>,
    input: HenrikMmrByNameRequest,
) -> AppResult<Value> {
    state.henrik.mmr_by_name(input).await
}

#[tauri::command]
pub async fn henrik_mmr_by_puuid(
    state: tauri::State<'_, AppState>,
    input: HenrikMmrByPuuidRequest,
) -> AppResult<Value> {
    state.henrik.mmr_by_puuid(input).await
}

#[tauri::command]
pub async fn henrik_mmr_history_by_name(
    state: tauri::State<'_, AppState>,
    input: HenrikMmrHistoryByNameRequest,
) -> AppResult<Value> {
    state.henrik.mmr_history_by_name(input).await
}

#[tauri::command]
pub async fn henrik_mmr_history_by_puuid(
    state: tauri::State<'_, AppState>,
    input: HenrikMmrHistoryByPuuidRequest,
) -> AppResult<Value> {
    state.henrik.mmr_history_by_puuid(input).await
}

#[tauri::command]
pub async fn henrik_vlr_events(state: tauri::State<'_, AppState>) -> AppResult<Value> {
    state.henrik.vlr_events().await
}

#[tauri::command]
pub async fn henrik_vlr_event_matches(
    state: tauri::State<'_, AppState>,
    input: HenrikVlrEntityRequest,
) -> AppResult<Value> {
    state.henrik.vlr_event_matches(input).await
}

#[tauri::command]
pub async fn henrik_vlr_match(
    state: tauri::State<'_, AppState>,
    input: HenrikVlrEntityRequest,
) -> AppResult<Value> {
    state.henrik.vlr_match(input).await
}

#[tauri::command]
pub async fn henrik_vlr_team(
    state: tauri::State<'_, AppState>,
    input: HenrikVlrEntityRequest,
) -> AppResult<Value> {
    state.henrik.vlr_team(input).await
}

#[tauri::command]
pub async fn henrik_vlr_team_matches(
    state: tauri::State<'_, AppState>,
    input: HenrikVlrEntityRequest,
) -> AppResult<Value> {
    state.henrik.vlr_team_matches(input).await
}

#[tauri::command]
pub async fn henrik_vlr_team_transactions(
    state: tauri::State<'_, AppState>,
    input: HenrikVlrEntityRequest,
) -> AppResult<Value> {
    state.henrik.vlr_team_transactions(input).await
}

#[tauri::command]
pub async fn henrik_vlr_player(
    state: tauri::State<'_, AppState>,
    input: HenrikVlrEntityRequest,
) -> AppResult<Value> {
    state.henrik.vlr_player(input).await
}

#[tauri::command]
pub async fn henrik_vlr_player_matches(
    state: tauri::State<'_, AppState>,
    input: HenrikVlrEntityRequest,
) -> AppResult<Value> {
    state.henrik.vlr_player_matches(input).await
}
