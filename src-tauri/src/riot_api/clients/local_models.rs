use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Entitlements {
    pub access_token: String,
    pub token: String,
    pub subject: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountAlias {
    pub active: bool,
    pub created_datetime: i64,
    pub game_name: String,
    pub summoner: bool,
    pub tag_line: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresenceData {
    pub basic: String,
    pub game_name: String,
    pub game_tag: String,
    pub name: String,
    pub pid: String,
    pub private: Option<String>,
    pub product: String,
    pub puuid: String,
    pub region: String,
    pub resource: String,
    pub state: String,
    pub summary: String,
    pub time: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Presence {
    pub presences: Vec<PresenceData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchPresenceData {
    pub match_map: Option<String>,
    pub provisioning_flow: Option<String>,
    pub queue_id: Option<String>,
    pub session_loop_state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartyPresenceData {
    pub custom_game_name: String,
    pub custom_game_team: String,
    pub is_party_cross_play_enabled: bool,
    pub is_party_owner: bool,
    pub max_party_size: i32,
    pub party_accessibility: String,
    pub party_client_version: String,
    pub party_id: Option<String>,
    pub party_l_f_m: bool,
    pub party_owner_match_map: String,
    pub party_owner_match_score_ally_team: i32,
    pub party_owner_match_score_enemy_team: i32,
    pub party_owner_provisioning_flow: String,
    pub party_owner_session_loop_state: String,
    pub party_precise_platform_types: i32,
    pub party_size: i32,
    pub party_state: String,
    pub party_version: i64,
    pub queue_entry_time: String,
    pub roster_id: String,
    pub tournament_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPresenceData {
    pub account_level: i32,
    pub competitive_tier: i32,
    pub leaderboard_position: i32,
    pub platform_override: String,
    pub player_card_id: String,
    pub player_title_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PremierPresenceData {
    pub division: i32,
    pub plating: i32,
    pub roster_id: String,
    pub roster_tag: String,
    pub roster_type: String,
    pub score: i32,
    pub show_aura: bool,
    pub show_plating: bool,
    pub show_tag: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePresence {
    is_idle: bool,
    is_valid: bool,
    match_presence_data: Option<MatchPresenceData>,
    max_party_size: i32,
    party_id: Option<String>,
    party_owner_match_score_ally_team: i32,
    party_owner_match_score_enemy_team: i32,
    party_presence_data: Option<PartyPresenceData>,
    party_size: i32,
    player_presence_data: Option<PlayerPresenceData>,
    premier_presence_data: Option<PremierPresenceData>,
    provisioning_flow: String,
    queue_id: String,
}
