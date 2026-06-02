use std::{env, fs, path::PathBuf};

use crate::errors::AppResult;

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub config_dir: PathBuf,
    pub settings_path: PathBuf,
    pub api_key_path: PathBuf,
    pub debug_log_path: PathBuf,
}

pub fn app_paths() -> AppResult<AppPaths> {
    let appdata = env::var("APPDATA")?;
    let config_dir = PathBuf::from(appdata).join("ValMonitor");

    Ok(AppPaths {
        settings_path: config_dir.join("settings.json"),
        api_key_path: config_dir.join("api.txt"),
        debug_log_path: config_dir.join("debug.log"),
        config_dir,
    })
}

pub fn ensure_config_dir() -> AppResult<AppPaths> {
    let paths = app_paths()?;
    fs::create_dir_all(&paths.config_dir)?;
    if !paths.debug_log_path.exists() {
        fs::File::create(&paths.debug_log_path)?;
    }
    Ok(paths)
}
