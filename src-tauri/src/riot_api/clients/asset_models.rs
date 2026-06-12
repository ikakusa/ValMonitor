use serde::{Deserialize, Serialize};

/*
    PlayerCard
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCardData {
    pub uuid: Option<String>,
    pub display_name: Option<String>,
    pub is_hidden_if_not_owned: Option<bool>,
    pub theme_uuid: Option<String>,
    pub display_icon: Option<String>,
    pub small_art: Option<String>,
    pub wide_art: Option<String>,
    pub large_art: Option<String>,
    pub asset_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerCard {
    pub status: u32,
    pub data: PlayerCardData,
}

/*
    End of PlayerCard
*/

/*
    RiotVersion
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RiotVersionData {
    pub manifest_id: String,
    pub branch: String,
    pub version: String,
    pub build_version: String,
    pub engine_version: String,
    pub riot_client_version: String,
    pub riot_client_build: String,
    pub build_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RiotVersion {
    pub status: u32,
    pub data: RiotVersionData,
}

/*
    End of RiotVersion
*/
