use std::fs;

use serde::{Deserialize, Serialize};

use crate::{app_config::ensure_config_dir, errors::AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikSettings {
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_auth_mode")]
    pub auth_mode: HenrikAuthMode,
    #[serde(default)]
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HenrikAuthMode {
    Header,
    Query,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikRuntimeSettings {
    pub settings_path: String,
    pub api_key_path: String,
    pub debug_log_path: String,
    pub base_url: String,
    pub auth_mode: HenrikAuthMode,
    pub has_api_key: bool,
}

impl Default for HenrikSettings {
    fn default() -> Self {
        Self {
            base_url: default_base_url(),
            auth_mode: default_auth_mode(),
            api_key: None,
        }
    }
}

pub fn load_settings() -> AppResult<HenrikSettings> {
    ensure_config_files()?;
    let paths = ensure_config_dir()?;
    let content = fs::read_to_string(&paths.settings_path)?;
    let mut settings = serde_json::from_str::<HenrikSettings>(&content).unwrap_or_default();

    if let Some(api_key) = read_api_key_file()? {
        settings.api_key = Some(api_key);
    }

    Ok(settings)
}

pub fn runtime_settings() -> AppResult<HenrikRuntimeSettings> {
    let settings = load_settings()?;
    let paths = ensure_config_dir()?;

    Ok(HenrikRuntimeSettings {
        settings_path: paths.settings_path.to_string_lossy().to_string(),
        api_key_path: paths.api_key_path.to_string_lossy().to_string(),
        debug_log_path: paths.debug_log_path.to_string_lossy().to_string(),
        base_url: settings.base_url,
        auth_mode: settings.auth_mode,
        has_api_key: settings.api_key.is_some(),
    })
}

pub fn save_api_key(api_key: String) -> AppResult<HenrikRuntimeSettings> {
    ensure_config_files()?;
    let paths = ensure_config_dir()?;
    fs::write(paths.api_key_path, api_key.trim())?;
    runtime_settings()
}

fn ensure_config_files() -> AppResult<()> {
    let paths = ensure_config_dir()?;

    if !paths.settings_path.exists() {
        let content = serde_json::to_string_pretty(&HenrikSettings::default())
            .expect("default Henrik settings must be serializable");
        fs::write(&paths.settings_path, content)?;
    }

    if !paths.api_key_path.exists() {
        // api.txt は秘密情報を分離するためのファイル。settings.json に key を直書きしなくても済む。
        fs::write(
            &paths.api_key_path,
            "# Henrik API key をこのファイルの 1 行目に貼り付けてください\n",
        )?;
    }

    Ok(())
}

fn read_api_key_file() -> AppResult<Option<String>> {
    let paths = ensure_config_dir()?;
    let content = fs::read_to_string(paths.api_key_path)?;
    let api_key = content
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#') && *line != "api")
        .map(ToString::to_string);

    Ok(api_key)
}

fn default_base_url() -> String {
    "https://api.henrikdev.xyz".to_string()
}

fn default_auth_mode() -> HenrikAuthMode {
    HenrikAuthMode::Header
}
