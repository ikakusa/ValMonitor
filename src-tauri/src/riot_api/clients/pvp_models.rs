use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/*
    Begin of MMR
*/
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MMRQueueSkillsSeasonData {
    pub season_i_d: String,
    pub number_of_wins: i32,
    pub number_of_wins_with_placements: i32,
    pub number_of_games: i32,
    pub rank: i32,
    pub capstone_wins: i32,
    pub leaderboard_rank: i32,
    pub competitive_tier: i32,
    pub ranked_rating: i32,
    pub wins_by_tier: Option<HashMap<String, i32>>,
    pub games_needed_for_rating: i32,
    pub total_wins_needed_for_rank: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MMRLatestCompetitiveUpdate {
    pub match_i_d: String,
    pub map_i_d: String,
    pub season_i_d: String,
    pub match_start_time: i64,
    pub tier_after_update: i32,
    pub tier_before_update: i32,
    pub ranked_rating_after_update: i32,
    pub ranked_rating_before_update: i32,
    pub ranked_rating_earned: i32,
    pub ranked_rating_performance_bonus: i32,
    pub a_f_k_penalty: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MMRQueueSkillsData {
    pub total_games_needed_for_rating: i32,
    pub total_games_needed_for_leaderboard: i32,
    pub current_season_games_needed_for_rating: i32,
    pub seasonal_info_by_season_i_d: Option<HashMap<String, MMRQueueSkillsSeasonData>>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MMR {
    pub version: i64,
    pub subject: String,
    pub new_player_experience_finished: bool,
    pub queue_skills: Option<HashMap<String, MMRQueueSkillsData>>,
    pub latest_competitive_update: Option<MMRLatestCompetitiveUpdate>,
    pub is_leaderboard_anonymized: bool,
    pub is_act_rank_badge_hidden: bool,
}

/*
    End of MMR
*/

/*
    Begin of PregameMatch
*/
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PregameSeasonalBadgeInfo {
    pub season_i_d: Option<String>,
    pub number_of_wins: i32,
    pub rank: i32,
    pub leaderboard_rank: i32,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PregamePlayerIdentity {
    pub subject: String,
    pub player_card_i_d: String,
    pub player_title_i_d: String,
    pub account_level: i32,
    pub preferred_level_border_i_d: Option<String>,
    pub incognito: bool,
    pub hide_account_level: bool,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PregamePlayers {
    pub subject: String,
    pub character_i_d: String,
    pub character_selection_state: String,
    pub competitive_tier: i32,
    pub player_identity: PregamePlayerIdentity,
    pub seasonal_badge_info: PregameSeasonalBadgeInfo,
    pub is_captain: bool,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PregameTeam {
    pub team_i_d: String,
    pub players: Vec<PregamePlayers>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PregameMatch {
    pub i_d: String,
    pub version: i64,
    pub teams: Vec<PregameTeam>,
    pub ally_team: Option<PregameTeam>,
    pub enemy_team: Option<PregameTeam>,
    pub enemy_team_size: i32,
    pub enemy_team_lock_count: i32,
    pub pregame_state: String,
    pub last_updated: String,
    pub map_i_d: String,
    pub map_select_step: i32,
    pub team1: String,
    pub game_pod_i_d: String,
    pub mode: String,
    pub voice_session_i_d: String,
    pub m_u_c_name: String,
    pub team_match_token: String,
    pub queue_i_d: String,
    pub provisioning_flow_i_d: String,
    pub is_ranked: bool,
    pub phase_time_remaining_n_s: i64,
    pub step_time_remaining_n_s: i64,
    #[serde(rename = "altModesFlagADA")]
    pub alt_modes_flag_a_d_a: bool,
}
/*
    End of PregameMatch
*/

/*
    Begin of PregamePlayer
*/
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct PregamePlayer {
    pub subject: String,
    pub match_i_d: String,
    pub version: i64
}
/*
    End of PregamePlayer
*/