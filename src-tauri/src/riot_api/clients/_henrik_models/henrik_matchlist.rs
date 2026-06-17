use serde::{Deserialize, Serialize};

/*
    Begin of MatchMetadata
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchMetadata {
    pub map: String,
    pub game_version: String,
    pub game_length: i64,
    pub game_start: i64,
    pub game_start_patched: String,
    pub rounds_played: i32,
    pub mode: String,
    pub mode_id: String,
    pub queue: String,
    pub season_id: String,
    pub platform: String,
    pub matchid: String,
    pub premier_info: MatchPremierInfo,
    pub region: String,
    pub cluster: String,
}
/*
    End of MatchMetadata
*/

/*
    Begin of MatchPremierInfo
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPremierInfo {
    pub tournament_id: Option<String>,
    pub matchup_id: Option<String>,
}
/*
    End of MatchPremierInfo
*/

/*
    Begin of MatchList
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchSessionPlaytime {
    pub minutes: i32,
    pub seconds: i32,
    pub milliseconds: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerAssetsCard {
    pub small: String,
    pub large: String,
    pub wide: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerAssetsAgent {
    pub small: String,
    pub full: String,
    pub bust: String,
    pub killfeed: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerAssets {
    pub card: MatchPlayerAssetsCard,
    pub agent: MatchPlayerAssetsAgent,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerBehaviorFF {
    pub incoming: f64,
    pub outgoing: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerBehavior {
    pub afk_rounds: f64,
    pub friendly_fire: MatchPlayerBehaviorFF,
    pub rounds_in_spawn: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerPlatformOS {
    pub name: String,
    pub version: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerPlatform {
    #[serde(rename = "type")]
    pub _type: String,
    pub os: MatchPlayerPlatformOS,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerAbilityCasts {
    pub c_cast: Option<i32>,
    pub q_cast: Option<i32>,
    pub e_cast: Option<i32>,
    pub x_cast: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerStats {
    pub score: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub bodyshots: i32,
    pub headshots: i32,
    pub legshots: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerEconomyValue {
    pub overall: i32,
    pub average: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayerEconomy {
    pub spent: MatchPlayerEconomyValue,
    pub loadout_value: MatchPlayerEconomyValue,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayer {
    pub puuid: String,
    pub name: String,
    pub tag: String,
    pub team: String,
    pub level: i32,
    pub character: String,
    pub currenttier: i32,
    pub currenttier_patched: String,
    pub player_card: String,
    pub player_title: String,
    pub party_id: String,
    pub session_playtime: MatchSessionPlaytime,
    pub assets: MatchPlayerAssets,
    pub behavior: MatchPlayerBehavior,
    pub platform: MatchPlayerPlatform,
    pub ability_casts: MatchPlayerAbilityCasts,
    pub stats: MatchPlayerStats,
    pub economy: MatchPlayerEconomy,
    pub damage_made: i32,
    pub damage_received: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchPlayers {
    pub all_players: Vec<MatchPlayer>,
    pub red: Vec<MatchPlayer>,
    pub blue: Vec<MatchPlayer>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchObserver {
    pub puuid: String,
    pub name: String,
    pub tag: String,
    pub platform: MatchPlayerPlatform,
    pub session_playtime: MatchSessionPlaytime,
    pub team: String,
    pub level: i32,
    pub player_card: String,
    pub player_title: String,
    pub party_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchCoach {
    pub puuid: String,
    pub team: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchTeaMRoasterCustomization {
    pub icon: String,
    pub image: String,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchTeamRoaster {
    pub members: Vec<String>,
    pub name: String,
    pub tag: String,
    pub customization: MatchTeaMRoasterCustomization,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchTeamData {
    pub has_won: bool,
    pub rounds_won: i32,
    pub rounds_lost: i32,
    pub roster: Option<MatchTeamRoaster>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchTeams {
    pub red: MatchTeamData,
    pub blue: MatchTeamData,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlantEventsPlantLocation {
    pub x: i32,
    pub y: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlantEventsPlantedBy {
    pub puuid: String,
    pub display_name: String,
    pub team: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlantEventsPlayerLocsOnPlant {
    pub player_puuid: String,
    pub player_display_name: String,
    pub player_team: String,
    pub location: Option<MatchRoundPlantEventsPlantLocation>,
    pub view_radians: f32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlantEvents {
    pub plant_location: Option<MatchRoundPlantEventsPlantLocation>,
    pub planted_by: Option<MatchRoundPlantEventsPlantedBy>,
    pub plant_site: Option<String>,
    pub plant_time_in_round: Option<i32>,
    pub player_locations_on_plant: Option<Vec<MatchRoundPlantEventsPlayerLocsOnPlant>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundDefuseEvents {
    pub defuse_location: Option<MatchRoundPlantEventsPlantLocation>,
    pub defused_by: Option<MatchRoundPlantEventsPlantedBy>,
    pub defuse_time_in_round: Option<i32>,
    pub player_locations_on_defuse: Option<Vec<MatchRoundPlantEventsPlayerLocsOnPlant>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundDamageEvents {
    pub receiver_puuid: String,
    pub receiver_display_name: String,
    pub receiver_team: String,
    pub bodyshots: i32,
    pub damage: i32,
    pub headshots: i32,
    pub legshots: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundKillEventsWeaponAssets {
    pub display_icon: Option<String>,
    pub killfeed_icon: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundKillEventsAssistants {
    pub assistant_puuid: String,
    pub assistant_display_name: String,
    pub assistant_team: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundKillEvents {
    pub kill_time_in_round: i32,
    pub kill_time_in_match: i32,
    pub killer_puuid: String,
    pub killer_display_name: String,
    pub killer_team: String,
    pub victim_puuid: String,
    pub victim_display_name: String,
    pub victim_team: String,
    pub victim_death_location: Option<MatchRoundPlantEventsPlantLocation>,
    pub damage_weapon_id: String,
    pub damage_weapon_name: Option<String>,
    pub damage_weapon_assets: MatchRoundKillEventsWeaponAssets,
    pub secondary_fire_mode: bool,
    pub player_locations_on_kill: Vec<MatchRoundPlantEventsPlayerLocsOnPlant>,
    pub assistants: Vec<MatchRoundKillEventsAssistants>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlayerStatsEconomyWeapon {
    pub id: Option<String>,
    pub name: Option<String>,
    pub assets: Option<MatchRoundKillEventsWeaponAssets>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlayerStatsEconomyArmorAssets {
    pub display_icon: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlayerStatsEconomyArmor {
    pub id: Option<String>,
    pub name: Option<String>,
    pub assets: MatchRoundPlayerStatsEconomyArmorAssets,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlayerStatsEconomy {
    pub loadout_value: i32,
    pub weapon: MatchRoundPlayerStatsEconomyWeapon,
    pub armor: MatchRoundPlayerStatsEconomyArmor,
    pub remaining: i32,
    pub spent: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRoundPlayerStats {
    pub ability_casts: MatchPlayerAbilityCasts,
    pub player_puuid: String,
    pub player_display_name: String,
    pub player_team: String,
    pub damage_events: Vec<MatchRoundDamageEvents>,
    pub damage: i32,
    pub bodyshots: i32,
    pub headshots: i32,
    pub legshots: i32,
    pub kill_events: Vec<MatchRoundKillEvents>,
    pub kills: i32,
    pub score: i32,
    pub economy: MatchRoundPlayerStatsEconomy,
    pub was_afk: bool,
    pub was_penalized: bool,
    pub stayed_in_spawn: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchRound {
    pub winning_team: String,
    pub end_type: String,
    pub bomb_planted: bool,
    pub bomb_defused: bool,
    pub plant_events: MatchRoundPlantEvents,
    pub defuse_events: MatchRoundDefuseEvents,
    pub player_stats: Vec<MatchRoundPlayerStats>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchListData {
    pub is_available: bool,
    pub metadata: Option<MatchMetadata>,
    pub players: Option<MatchPlayers>,
    pub observers: Option<Vec<MatchObserver>>,
    pub coaches: Option<Vec<MatchCoach>>,
    pub teams: Option<MatchTeams>,
    pub rounds: Option<Vec<MatchRound>>,
    pub kills: Option<Vec<MatchRoundKillEvents>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchList {
    pub status: i32,
    pub data: Vec<MatchListData>,
}
/*
    End of MatchList
*/