// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod commands;
pub mod riot_api;

use crate::riot_api::core::VALORANT_API;
// use crate::commands::henrik_commands;
use crate::commands::{asset_commands, local_commands, pvp_commands, henrik_commands};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    VALORANT_API.clone().monitor_lockfile();
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            local_commands::get_presence,
            local_commands::get_my_presence,
            local_commands::get_private_presence,
            local_commands::get_gamestate,
            local_commands::is_api_initialized,
            local_commands::get_puuid,
            local_commands::get_full_username,
            local_commands::get_region,

            pvp_commands::get_player_mmr,
            pvp_commands::get_current_pregame,
            
            asset_commands::get_playercard_by_id,
            
            henrik_commands::get_player_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
