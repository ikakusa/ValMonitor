<<<<<<< HEAD
#![allow(linker_messages)]

mod app_config;
mod commands;
mod discord_rpc;
mod errors;
mod riot;
mod henrik;
mod http_log;
mod models;
mod state;

use app_config::ensure_config_dir;
use commands::discord::{discord_rpc_clear, discord_rpc_set_activity};
use commands::henrik::{
    henrik_account_by_name, henrik_account_by_puuid, henrik_content, henrik_crosshair,
    henrik_esports_schedule, henrik_get_settings, henrik_leaderboard, henrik_match_by_id,
    henrik_matches_by_name, henrik_matches_by_puuid, henrik_mmr_by_name, henrik_mmr_by_puuid,
    henrik_mmr_history_by_name, henrik_mmr_history_by_puuid, henrik_save_api_key,
    henrik_vlr_event_matches, henrik_vlr_events, henrik_vlr_match, henrik_vlr_player,
    henrik_vlr_player_matches, henrik_vlr_team, henrik_vlr_team_matches,
    henrik_vlr_team_transactions,
};
use log::LevelFilter;
use state::AppState;
use tauri_plugin_log::{Target, TargetKind};
=======
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod riot;
use riot::ValorantAPI;
use serde_json;
use serde_json::Value;
use std::sync::Arc;
use std::sync::{Mutex, OnceLock};

static VALORANT_API: OnceLock<Mutex<Arc<ValorantAPI>>> = OnceLock::new();

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_player_mmr(uid: String) -> Result<Value, String> {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();

    api.get_mmr(uid.as_str()).await
}

#[tauri::command]
async fn get_presence() -> String {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();
    match api.get_presence().await {
        Ok(json) => {
            return String::from(serde_json::to_string(&json).unwrap());
        }
        Err(_err) => {
            return String::from("Invalid Json");
        }
    }
}

#[tauri::command]
async fn get_auth_userinfo() -> String {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();
    match api.get_userinfo().await {
        Ok(json) => {
            return String::from(serde_json::to_string(&json).unwrap());
        }
        Err(_err) => {
            return String::from("Invalid Json");
        }
    }
}

#[tauri::command]
async fn get_my_presence() -> Result<Value, String> {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();
    api.get_my_presence().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_private_presence() -> Result<Value, String> {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();
    api.get_private_presence().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_gamestate() -> Result<String, String> {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();

    api.get_gamestate().await.map_err(|e| e.to_string())
}
#[tauri::command]
async fn is_api_initialized() -> bool {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();
    return api.is_initialized().await;
}

#[tauri::command]
async fn get_full_username() -> String {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();

    return format!(
        "{}#{}",
        *api.name.lock().unwrap(),
        *api.tag_line.lock().unwrap()
    );
}

#[tauri::command]
async fn get_puuid() -> String {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();

    return api.puuid.lock().unwrap().clone();
}

#[tauri::command]
async fn get_playercard_by_id(id: String) -> Result<Value, String> {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();

    api.get_playercard_by_id(id).await
}

#[tauri::command]
async fn get_region() -> Result<String, String> {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();

    api.get_region().await
}
>>>>>>> 3ea16ef1c38eb97cc19d82291190497bca6d9c9e

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let api: Arc<ValorantAPI> = Arc::new(ValorantAPI::new());
    let _ = VALORANT_API.set(Mutex::new(api));
    VALORANT_API
        .get()
        .expect("VALORANT API IS NOT INITIALIZED")
        .lock()
        .unwrap()
        .clone()
        .monitor_lockfile();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
<<<<<<< HEAD
            commands::riot_command::get_presence,
            commands::riot_command::get_all_presences,
            commands::riot_command::get_my_presence,
            commands::riot_command::get_auth_userinfo,
            commands::riot_command::get_private_presence,
            commands::riot_command::get_gamestate,
            commands::riot_command::is_api_initialized,
            commands::riot_command::get_puuid,
            commands::riot_command::get_full_username,
            commands::riot_command::get_playercard_by_id,
            commands::riot_command::get_region,
            commands::riot_command::get_player_mmr,
            commands::riot_command::get_player_loadout,
            commands::riot_command::get_storefront,
            commands::riot_command::get_current_match,
            henrik_get_settings,
            henrik_save_api_key,
            henrik_account_by_name,
            henrik_account_by_puuid,
            henrik_content,
            henrik_crosshair,
            henrik_esports_schedule,
            henrik_leaderboard,
            henrik_matches_by_name,
            henrik_matches_by_puuid,
            henrik_match_by_id,
            henrik_mmr_by_name,
            henrik_mmr_by_puuid,
            henrik_mmr_history_by_name,
            henrik_mmr_history_by_puuid,
            henrik_vlr_events,
            henrik_vlr_event_matches,
            henrik_vlr_match,
            henrik_vlr_team,
            henrik_vlr_team_matches,
            henrik_vlr_team_transactions,
            henrik_vlr_player,
            henrik_vlr_player_matches,
            discord_rpc_set_activity,
            discord_rpc_clear
=======
            greet,
            get_presence,
            get_my_presence,
            get_auth_userinfo,
            get_private_presence,
            get_gamestate,
            is_api_initialized,
            get_puuid,
            get_full_username,
            get_playercard_by_id,
            get_region,
            get_player_mmr
>>>>>>> 3ea16ef1c38eb97cc19d82291190497bca6d9c9e
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
