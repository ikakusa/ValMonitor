use serde::{Deserialize, Serialize};


/*
    PlayerCard
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCardData {
    pub uuid: String,
    pub display_name: String,
    pub is_hidden_if_not_owned: bool,
    pub theme_uuid: String,
    pub display_icon: String,
    pub small_art: String,
    pub wide_art: String,
    pub large_art: String,
    pub asset_path: String,
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
#[serde(rename_all = "camelCase")]
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
