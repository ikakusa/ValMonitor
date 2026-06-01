// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod riot;
use riot::ValorantAPI;
use std::sync::Arc;
use std::sync::{Mutex, OnceLock};

static VALORANT_API: OnceLock<Mutex<Arc<ValorantAPI>>> = OnceLock::new();

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
async fn get_my_presence() -> String {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();
    match api.get_my_presence().await {
        Ok(json) => return String::from(serde_json::to_string(&json).unwrap()),
        Err(_err) => {
            return String::from("Invalid Json");
        }
    }
}

#[tauri::command]
async fn get_private_presence() -> String {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();
    match api.get_private_presence().await {
        Ok(json) => json.to_string(),
        Err(_err) => {
            return String::from("Invalid Json");
        }
    }
}

#[tauri::command]
async fn get_gamestate() -> String {
    let api = VALORANT_API
        .get()
        .expect("Valorant API is not initialized!!!")
        .lock()
        .unwrap()
        .clone();
    match api.get_gamestate().await {
        Ok(json) => return json,
        Err(_err) => {
            return String::from("NONE");
        }
    }
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

    return format!("{}#{}", *api.name.lock().unwrap(), *api.tag_line.lock().unwrap());
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
            get_full_username
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
