pub use super::_henrik_models::henrik_matchlist;
use serde::{Deserialize, Serialize};

/*
    Begin of Account
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountData {
    pub puuid: String,
    pub region: String,
    pub account_level: i32,
    pub name: String,
    pub tag: String,
    pub card: String,
    pub title: String,
    pub platforms: Vec<String>,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub status: i32,
    pub data: AccountData,
}
/*
    End of Account
*/

/*
    Begin of PlayerStats
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStats {
    pub matches: i32,
    pub rounds_played: i32,
    pub total_damage: i32,
    pub avg_damage_round: f32,
    pub kills: i32,
    pub avg_kills: f32,
    pub avg_deaths: f32,
    pub min_kills: i32,
    pub min_deaths: i32,
    pub max_kills: i32,
    pub max_deaths: i32,
    pub deaths: i32,
    pub kdr: f32,
    pub win_rate: f32,
    pub total_score: f32,
    pub avg_score: f32,
    pub wins: i32,
    pub loses: i32,
    pub headshot: i32,
    pub bodyshot: i32,
    pub legshot: i32,
    pub total_shot: i32,
    pub hs_percent: f32,
    pub bs_percent: f32,
    pub ls_percent: f32,
}
/*
    End of PlayerStats
*/