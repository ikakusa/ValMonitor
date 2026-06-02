use serde::Deserialize;
use serde_json::Value;

use crate::errors::AppResult;

use super::client::{query, segment};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikAccountByNameRequest {
    pub name: String,
    pub tag: String,
    pub force: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikAccountByPuuidRequest {
    pub puuid: String,
    pub force: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikContentRequest {
    pub locale: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikCrosshairRequest {
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikEsportsScheduleRequest {
    pub region: Option<String>,
    pub league: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikLeaderboardRequest {
    pub region: String,
    pub platform: String,
    pub season: Option<String>,
    pub size: Option<u32>,
    pub page: Option<u32>,
    pub start_index: Option<u32>,
    pub name: Option<String>,
    pub tag: Option<String>,
    pub puuid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikMatchesByNameRequest {
    pub region: String,
    pub platform: String,
    pub name: String,
    pub tag: String,
    pub mode: Option<String>,
    pub map: Option<String>,
    pub size: Option<u32>,
    pub start: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikMatchesByPuuidRequest {
    pub region: String,
    pub platform: String,
    pub puuid: String,
    pub mode: Option<String>,
    pub map: Option<String>,
    pub size: Option<u32>,
    pub start: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikMatchRequest {
    pub region: String,
    pub match_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikMmrByNameRequest {
    pub region: String,
    pub platform: String,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikMmrByPuuidRequest {
    pub region: String,
    pub platform: String,
    pub puuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikMmrHistoryByNameRequest {
    pub region: String,
    pub platform: String,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikMmrHistoryByPuuidRequest {
    pub region: String,
    pub platform: String,
    pub puuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HenrikVlrEntityRequest {
    pub id: String,
}

impl super::HenrikClient {
    pub async fn account_by_name(&self, input: HenrikAccountByNameRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v2/account/{}/{}",
                segment(&input.name),
                segment(&input.tag)
            ),
            query(vec![("force", input.force.map(|value| value.to_string()))]),
        )
        .await
    }

    pub async fn account_by_puuid(&self, input: HenrikAccountByPuuidRequest) -> AppResult<Value> {
        self.get(
            format!("/valorant/v2/by-puuid/account/{}", segment(&input.puuid)),
            query(vec![("force", input.force.map(|value| value.to_string()))]),
        )
        .await
    }

    pub async fn content(&self, input: HenrikContentRequest) -> AppResult<Value> {
        self.get(
            "/valorant/v1/content",
            query(vec![("locale", input.locale)]),
        )
        .await
    }

    pub async fn crosshair(&self, input: HenrikCrosshairRequest) -> AppResult<Value> {
        self.get(
            "/valorant/v1/crosshair/generate",
            query(vec![("id", input.id)]),
        )
        .await
    }

    pub async fn esports_schedule(&self, input: HenrikEsportsScheduleRequest) -> AppResult<Value> {
        self.get(
            "/valorant/v1/esports/schedule",
            query(vec![("region", input.region), ("league", input.league)]),
        )
        .await
    }

    pub async fn leaderboard(&self, input: HenrikLeaderboardRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v3/leaderboard/{}/{}",
                segment(&input.region),
                segment(&input.platform)
            ),
            query(vec![
                ("season", input.season),
                ("size", input.size.map(|value| value.to_string())),
                ("page", input.page.map(|value| value.to_string())),
                (
                    "start_index",
                    input.start_index.map(|value| value.to_string()),
                ),
                ("name", input.name),
                ("tag", input.tag),
                ("puuid", input.puuid),
            ]),
        )
        .await
    }

    pub async fn matches_by_name(&self, input: HenrikMatchesByNameRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v4/matches/{}/{}/{}/{}",
                segment(&input.region),
                segment(&input.platform),
                segment(&input.name),
                segment(&input.tag)
            ),
            matchlist_query(input.mode, input.map, input.size, input.start),
        )
        .await
    }

    pub async fn matches_by_puuid(&self, input: HenrikMatchesByPuuidRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v4/by-puuid/matches/{}/{}/{}",
                segment(&input.region),
                segment(&input.platform),
                segment(&input.puuid)
            ),
            matchlist_query(input.mode, input.map, input.size, input.start),
        )
        .await
    }

    pub async fn match_by_id(&self, input: HenrikMatchRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v4/match/{}/{}",
                segment(&input.region),
                segment(&input.match_id)
            ),
            query(vec![]),
        )
        .await
    }

    pub async fn mmr_by_name(&self, input: HenrikMmrByNameRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v3/mmr/{}/{}/{}/{}",
                segment(&input.region),
                segment(&input.platform),
                segment(&input.name),
                segment(&input.tag)
            ),
            query(vec![]),
        )
        .await
    }

    pub async fn mmr_by_puuid(&self, input: HenrikMmrByPuuidRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v3/by-puuid/mmr/{}/{}/{}",
                segment(&input.region),
                segment(&input.platform),
                segment(&input.puuid)
            ),
            query(vec![]),
        )
        .await
    }

    pub async fn mmr_history_by_name(
        &self,
        input: HenrikMmrHistoryByNameRequest,
    ) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v2/mmr-history/{}/{}/{}/{}",
                segment(&input.region),
                segment(&input.platform),
                segment(&input.name),
                segment(&input.tag)
            ),
            query(vec![]),
        )
        .await
    }

    pub async fn mmr_history_by_puuid(
        &self,
        input: HenrikMmrHistoryByPuuidRequest,
    ) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v2/by-puuid/mmr-history/{}/{}/{}",
                segment(&input.region),
                segment(&input.platform),
                segment(&input.puuid)
            ),
            query(vec![]),
        )
        .await
    }

    pub async fn vlr_events(&self) -> AppResult<Value> {
        self.get("/valorant/v2/esports/vlr/events", query(vec![]))
            .await
    }

    pub async fn vlr_event_matches(&self, input: HenrikVlrEntityRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v2/esports/vlr/events/{}/matches",
                segment(&input.id)
            ),
            query(vec![]),
        )
        .await
    }

    pub async fn vlr_match(&self, input: HenrikVlrEntityRequest) -> AppResult<Value> {
        self.get(
            format!("/valorant/v2/esports/vlr/matches/{}", segment(&input.id)),
            query(vec![]),
        )
        .await
    }

    pub async fn vlr_team(&self, input: HenrikVlrEntityRequest) -> AppResult<Value> {
        self.get(
            format!("/valorant/v2/esports/vlr/teams/{}", segment(&input.id)),
            query(vec![]),
        )
        .await
    }

    pub async fn vlr_team_matches(&self, input: HenrikVlrEntityRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v2/esports/vlr/teams/{}/matches",
                segment(&input.id)
            ),
            query(vec![]),
        )
        .await
    }

    pub async fn vlr_team_transactions(&self, input: HenrikVlrEntityRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v2/esports/vlr/teams/{}/transactions",
                segment(&input.id)
            ),
            query(vec![]),
        )
        .await
    }

    pub async fn vlr_player(&self, input: HenrikVlrEntityRequest) -> AppResult<Value> {
        self.get(
            format!("/valorant/v2/esports/vlr/players/{}", segment(&input.id)),
            query(vec![]),
        )
        .await
    }

    pub async fn vlr_player_matches(&self, input: HenrikVlrEntityRequest) -> AppResult<Value> {
        self.get(
            format!(
                "/valorant/v2/esports/vlr/players/{}/matches",
                segment(&input.id)
            ),
            query(vec![]),
        )
        .await
    }
}

fn matchlist_query(
    mode: Option<String>,
    map: Option<String>,
    size: Option<u32>,
    start: Option<u32>,
) -> std::collections::BTreeMap<String, String> {
    query(vec![
        ("mode", mode),
        ("map", map),
        ("size", size.map(|value| value.to_string())),
        ("start", start.map(|value| value.to_string())),
    ])
}
