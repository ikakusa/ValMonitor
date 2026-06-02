use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Deserialize)]
pub struct ValorantApiDataResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValorantVersionData {
    pub riot_client_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCardData {
    pub uuid: Option<String>,
    pub display_name: Option<String>,
    pub display_icon: Option<String>,
    pub small_art: Option<String>,
    pub wide_art: Option<String>,
    pub large_art: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
