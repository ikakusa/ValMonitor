use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceResponse {
    pub presences: Vec<PresenceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceEntry {
    pub puuid: String,
    pub private: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExternalSession {
    #[serde(rename = "launchConfiguration")]
    pub launch_configuration: LaunchConfiguration,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LaunchConfiguration {
    pub arguments: Vec<String>,
}

pub type ExternalSessionsResponse = HashMap<String, Value>;
