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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
