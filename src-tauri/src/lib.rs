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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let paths = ensure_config_dir().expect("failed to prepare ValMonitor config directory");
    let state = AppState::new();
    state.start_background_tasks();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(LevelFilter::Debug)
                .level_for("reqwest", LevelFilter::Warn)
                .level_for("hyper", LevelFilter::Warn)
                .level_for("h2", LevelFilter::Warn)
                .level_for("rustls", LevelFilter::Warn)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Folder {
                        path: paths.config_dir.clone(),
                        file_name: Some("debug".into()),
                    }),
                ])
                .build(),
        )
        .manage(state)
        .invoke_handler(tauri::generate_handler![
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
