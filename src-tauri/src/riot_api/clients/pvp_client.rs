use crate::riot_api::{
    clients::{
        asset_client::AssetClient,
        pvp_models::{Match, MatchPlayer, PregameMatch, PregamePlayer, MMR},
    },
    notify::NotifyStruct,
    response::{RequestError, RiotResponse},
    shared_data::SharedGameData,
};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc, Mutex},
    time::Duration,
};

pub struct PvPClient {
    client: Mutex<Client>,
    initialized: NotifyStruct,
    initialized_http: NotifyStruct,
    shared: Arc<SharedGameData>,
    shards: HashMap<String, String>,
}

impl PvPClient {
    pub async fn send_request(
        &self,
        path: &str,
        use_glz: bool,
        skip_reset_check: bool,
    ) -> Result<RiotResponse, RequestError> {
        while !self.initialized_http.value.load(Ordering::Acquire) {
            self.initialized_http.notifier.notified().await;
        }

        if !skip_reset_check {
            while self.shared.should_reset() {
                let notified = self.shared.need_reset.notifier.notified();
                if !self.shared.should_reset() {
                    break;
                }
                notified.await;
            }
        }

        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 3 {
                return Err(RequestError::Unknown("Max retries exceeded".into()));
            }
            let region = self.shared.get_region();

            let pre = if use_glz {
                format!("glz-{}-1.{}", self.shards.get(&region).unwrap(), region)
            } else {
                format!("pd.{}", region)
            };

            let url = format!("https://{}.a.pvp.net{}", pre, path);
            println!("[PvPClient::send_request] {} (Attempt: {})", url, attempts);

            let client = { self.client.lock().unwrap().clone() };
            let response = client.get(url).send().await?;

            let status = response.status();
            let bytes = response.bytes().await?;

            if !status.is_success() {
                self.shared.order_reset(true);
                if !skip_reset_check {
                    while self.shared.should_reset() {
                        let notified = self.shared.need_reset.notifier.notified();
                        if !self.shared.should_reset() {
                            break;
                        }
                        notified.await;
                    }
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
                } else {
                    return Err(RequestError::Unknown("Failed to send request".into()));
                }
            }

            break Ok(RiotResponse {
                bytes: bytes,
                status: status,
            });
        }
    }
    pub async fn get_mmr(&self, puuid: &str, skip: bool) -> Result<MMR, RequestError> {
        let res = self
            .send_request(format!("/mmr/v1/players/{puuid}").as_str(), false, skip)
            .await?
            .get_json::<MMR>()?;
        Ok(res)
    }
    pub async fn get_pregame_match(
        &self,
        match_id: &str,
        skip: bool,
    ) -> Result<PregameMatch, RequestError> {
        let res = self
            .send_request(
                format!("/pregame/v1/matches/{}", match_id).as_str(),
                true,
                skip,
            )
            .await?
            .get_json::<PregameMatch>()?;
        Ok(res)
    }
    pub async fn get_pregame_player(
        &self,
        puuid: &str,
        skip: bool,
    ) -> Result<PregamePlayer, RequestError> {
        let res = self
            .send_request(
                format!("/pregame/v1/players/{}", puuid).as_str(),
                true,
                skip,
            )
            .await?
            .get_json::<PregamePlayer>()?;
        Ok(res)
    }
    pub async fn get_match_data(
        &self,
        match_id: &str,
        skip: bool,
    ) -> Result<Match, RequestError> {
        let res = self
            .send_request(
                format!("/core-game/v1/matches/{}", match_id).as_str(),
                true,
                skip,
            )
            .await?
            .get_json::<Match>()?;
        Ok(res)
    }
    pub async fn get_match_player(
        &self,
        puuid: &str,
        skip: bool,
    ) -> Result<MatchPlayer, RequestError> {
        let res = self
            .send_request(
                format!("/core-game/v1/players/{}", puuid).as_str(),
                true,
                skip,
            )
            .await?
            .get_json::<MatchPlayer>()?;
        Ok(res)
    }
    pub async fn get_current_game(&self, skip: bool) -> Result<Match, RequestError> {
        let puuid = { self.shared.get_user().clone().puuid };
        let match_player = self.get_match_player(&puuid, skip).await?;
        let match_data = self.get_match_data(match_player.match_i_d.as_str(), skip).await?;
        Ok(match_data)
    }
    pub async fn get_current_pregame(&self, skip: bool) -> Result<PregameMatch, RequestError> {
        let puuid = { self.shared.get_user().clone().puuid };
        let pregame_player = self.get_pregame_player(&puuid, skip).await?;
        let pregame_match = self
            .get_pregame_match(pregame_player.match_i_d.as_str(), skip)
            .await?;
        Ok(pregame_match)
    }
    pub async fn build(&self) -> Result<(), RequestError> {
        let auth_token = { self.shared.get_auth().clone() };
        let basic_authorization = format!("Bearer {}", auth_token.access_token);

        *self.shared.get_auth() = auth_token.clone();

        let mut bheader = HeaderMap::new();
        bheader.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(basic_authorization.as_str()).unwrap(),
        );
        bheader.insert(
            "X-Riot-Entitlements-JWT",
            HeaderValue::from_str(auth_token.entitlements_token.as_str()).unwrap(),
        );
        bheader.insert(
            "X-Riot-ClientPlatform",
            HeaderValue::from_str("ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9").unwrap(),
        );

        let v = AssetClient::get_version().await?.data.riot_client_version;
        bheader.insert(
            "X-Riot-ClientVersion",
            HeaderValue::from_str(v.as_str()).unwrap(),
        );

        let new_client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(bheader)
            .build()
            .unwrap();

        *self.client.lock().unwrap() = new_client;

        self.initialized_http.set_and_notify(true);
        self.initialized.set_and_notify(true);
        Ok(())
    }
    pub fn new(shared: Arc<SharedGameData>) -> Self {
        let mut shards = HashMap::new();
        shards.insert("ap".into(), "ap".into());
        shards.insert("kr".into(), "kr".into());
        shards.insert("latam".into(), "na".into());
        shards.insert("br".into(), "na".into());
        shards.insert("na".into(), "na".into());
        shards.insert("eu".into(), "eu".into());

        Self {
            shared: shared,
            initialized: NotifyStruct::default(),
            initialized_http: NotifyStruct::default(),
            client: Mutex::new(reqwest::Client::new()),
            shards: shards,
        }
    }
}
