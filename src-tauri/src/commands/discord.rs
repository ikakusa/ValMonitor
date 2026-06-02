use crate::{discord_rpc::DiscordRpcActivityInput, errors::AppResult, state::AppState};

#[tauri::command]
pub async fn discord_rpc_set_activity(
    state: tauri::State<'_, AppState>,
    input: DiscordRpcActivityInput,
) -> AppResult<()> {
    state.discord_rpc.set_activity(input)
}

#[tauri::command]
pub async fn discord_rpc_clear(state: tauri::State<'_, AppState>) -> AppResult<()> {
    state.discord_rpc.clear()
}
