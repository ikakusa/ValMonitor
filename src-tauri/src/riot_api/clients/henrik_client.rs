use crate::riot_api::{
    clients::henrik_models::{henrik_matchlist::MatchList, Account, PlayerStats},
    notify::NotifyStruct,
    response::{RequestError, RiotResponse},
    shared_data::SharedGameData,
};
use reqwest::{header, Client};
use std::{
    fs,
    sync::{atomic::Ordering, Arc, Mutex},
    time::Duration,
};
use tauri::{http::{HeaderMap, HeaderValue}};

#[derive(Default)]
pub struct HenrikClient {
    client: Mutex<Client>,
    pub initialized: NotifyStruct,
    initialized_http: NotifyStruct,
    pub shared: Arc<SharedGameData>,
    pub apikey: Mutex<String>,
}

impl HenrikClient {
    pub async fn read_apikey(&self) -> Result<String, std::io::Error> {
        let path = self.shared.app_data_path.join("apikey");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        if !path.exists() {
            fs::write(&path, "")?;
        }

        let content = fs::read_to_string(path)?;
        Ok(content)
    }
    pub async fn send_request(&self, path: &str) -> Result<RiotResponse, RequestError> {
        while !self.initialized_http.value.load(Ordering::Acquire) {
            self.initialized_http.notifier.notified().await;
        }
        loop {
            println!("[HenrikClient::send_request] {}", path);
            let client = { self.client.lock().unwrap().clone() };
            let res = client
                .get(format!("https://api.henrikdev.xyz/valorant{}", path))
                .send()
                .await?;
            let status = res.status();
            if !status.is_success() {
                let _ = self.build().await;
                let mut wait_time = 5;
                if status == 429 {
                    wait_time = 60;
                }
                tokio::time::sleep(Duration::from_secs(wait_time)).await;
                continue;
            }

            let bytes = res.bytes().await?;
            break Ok(RiotResponse {
                bytes: bytes,
                status: status,
            });
        }
    }
    pub async fn get_account_by_id(&self, puuid: &str) -> Result<Account, RequestError> {
        Ok(self
            .send_request(format!("/v2/by-puuid/account/{}", puuid).as_str())
            .await?
            .get_json::<Account>()?)
    }
    pub async fn get_matchlist_by_puuid<T: Into<Option<String>>>(
        &self,
        puuid: &str,
        queue: T,
    ) -> Result<MatchList, RequestError> {
        let region = { self.shared.get_user().clone().region };
        let q = queue.into();
        let url = if q.is_none() {
            format!("/v3/by-puuid/matches/{}/{}?size=10", region, puuid)
        } else {
            format!(
                "/v3/by-puuid/matches/{}/{}?mode={}&size=10",
                region,
                puuid,
                q.unwrap()
            )
        };
            //     Clipboard::new().unwrap().set_text(serde_json::to_string(&self
            // .send_request(url.as_str())
            // .await?.get_json::<Value>()?["data"].as_array().unwrap()).unwrap());
        Ok(self
            .send_request(url.as_str())
            .await?
            .get_json::<MatchList>()?)
    }
    pub async fn get_player_stats_by_id(&self, puuid: &str) -> Result<PlayerStats, RequestError> {
        let matchlist = self
            .get_matchlist_by_puuid(puuid, String::from("competitive"))
            .await?;

        let mut total_kills = 0;
        let mut total_deaths = 0;
        let mut rounds = 0;
        let mut wins = 0;
        let mut loses = 0;
        let mut damages = 0;
        let mut scores = 0.0 as f32;
        let mut min_kills = 0;
        let mut max_kills = 0;
        let mut min_deaths = 0;
        let mut max_deaths = 0;
        let mut headshots = 0;
        let mut bodyshots = 0;
        let mut legshots = 0;
        let mut total_shots = 0;

        let total_matches = matchlist.data.len() as i32;
        for match_data in matchlist.data.iter() {
            if !match_data.is_available {
                continue;
            }
            let metadata = &&match_data.metadata.as_ref().unwrap();
            let teams = &match_data.teams.as_ref().unwrap();
            rounds += metadata.rounds_played;
            let my_stats = match_data
                .players
                .as_ref()
                .unwrap()
                .all_players
                .iter()
                .find(|p| p.puuid == puuid)
                .unwrap();
            let my_team = if my_stats.team == "Red" {
                &teams.red
            } else {
                &teams.blue
            };
            let has_won = my_team.has_won;
            let stats = &my_stats.stats;
            let _kills = stats.kills;
            let _deaths = stats.deaths;
            wins += has_won as i32;
            loses += (has_won == false) as i32;
            total_kills += _kills;
            total_deaths += _deaths;
            damages += my_stats.damage_made;
            scores += stats.score as f32 / metadata.rounds_played as f32;
            min_kills = min_kills.min(_kills);
            max_kills = max_kills.max(_kills);

            min_deaths = min_deaths.min(_deaths);
            max_deaths = max_deaths.max( _deaths);

            headshots += stats.headshots;
            bodyshots += stats.bodyshots;
            legshots += stats.legshots;
        }

        total_shots += headshots + legshots + bodyshots;

        Ok(PlayerStats {
            kdr: total_kills as f32 / total_deaths.max(1) as f32,
            kills: total_kills,
            deaths: total_deaths,
            win_rate: (wins as f32 / total_matches.max(1) as f32) * 100.0,
            wins: wins,
            loses: loses,
            total_damage: damages,
            matches: total_matches,
            rounds_played: rounds,
            avg_damage_round: damages as f32 / rounds.max(1) as f32,
            total_score: scores,
            avg_score: scores as f32 / total_matches.max(1) as f32,
            avg_kills: total_kills as f32 / total_matches.max(1) as f32,
            avg_deaths: total_deaths as f32 / total_matches.max(1) as f32,
            min_deaths: min_deaths,
            min_kills: min_kills,
            max_deaths: max_deaths,
            max_kills: max_kills,
            total_shot: total_shots,
            hs_percent: (headshots as f32 / total_shots.max(1) as f32) * 100.0,
            bs_percent: (bodyshots as f32 / total_shots.max(1) as f32) * 100.0,
            ls_percent: (legshots as f32 / total_shots.max(1) as f32) * 100.0,
            headshot: headshots,
            bodyshot: bodyshots,
            legshot: legshots
        })
    }
    pub async fn build(&self) -> Result<(), RequestError> {
        loop {
            let api_key = self
                .read_apikey()
                .await
                .map_err(|e| RequestError::Unknown(e.to_string()))?;
            *self.apikey.lock().unwrap() = api_key.clone();
            if api_key.len() < 41 {
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }

            self.initialized_http.set_and_notify(false);
            self.initialized.set_and_notify(false);

            let mut header = HeaderMap::new();
            header.insert(
                header::AUTHORIZATION,
                HeaderValue::from_str(api_key.as_str()).unwrap(),
            );
            *self.client.lock().unwrap() =
                Client::builder().default_headers(header).build().unwrap();
            self.initialized_http.set_and_notify(true);
            self.initialized.set_and_notify(true);
            break Ok(());
        }
    }
    pub fn new(shared: Arc<SharedGameData>) -> Self {
        let mut ins = Self::default();
        ins.shared = shared;
        ins
    }
}
