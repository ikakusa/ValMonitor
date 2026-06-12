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
