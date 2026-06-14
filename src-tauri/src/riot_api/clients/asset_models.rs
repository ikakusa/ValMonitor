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

/*
    Begin of Agent
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct AgentData {
    pub uuid: String,
    pub display_name: String,
    pub description: String,
    pub developer_name: String,
    pub release_date: String,
    pub display_icon: String,
    pub display_icon_small: String,
    pub bust_portrait: Option<String>,
    pub full_portrait: Option<String>,
    pub full_portrait_v2: Option<String>,
    pub killfeed_portrait: Option<String>,
    pub minimap_portrait: Option<String>,
    pub home_screen_promo_tile_image: Option<String>,
    pub background: Option<String>,
    pub is_full_portrait_right_facing: bool,
    pub is_playable_character: bool,
    pub is_available_for_test: bool,
    pub is_base_content: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
    pub status: u32,
    pub data: AgentData
}
/*
    End of Agent
*/