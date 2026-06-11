use super::local_models::{AccountAlias, Entitlements};
use crate::riot_api::clients::local_models::{Presence, PresenceData, PrivatePresence};
use crate::riot_api::data::auth_token::AuthToken;
use crate::riot_api::data::user_data::UserData;
use crate::riot_api::notify::NotifyStruct;
use crate::riot_api::response::{RequestError, RiotResponse};
use crate::riot_api::shared_data::SharedGameData;
use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header::AUTHORIZATION, Client};
use serde_json::Value;
use std::env;
use std::fs;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::time::Duration;

pub struct LocalClient {
    client: Mutex<Client>,
    pub initialized: NotifyStruct,
    initialized_http: NotifyStruct,
    shared: Arc<SharedGameData>,
}

impl LocalClient {
    pub async fn send_request(
        &self,
        path: &str,
        skip_reset_check: bool,
    ) -> Result<RiotResponse, RequestError> {
        let result = async {
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

            loop {
                let port = {
                    let auth = self.shared.get_auth();
                    auth.lockfile_port.clone()
                };
                let addr = format!("127.0.0.1:{}", port);
                let wa = addr
                    .to_socket_addrs()
                    .unwrap()
                    .find(|x| (*x).is_ipv4())
                    .unwrap();
                while !TcpStream::connect(wa).is_ok() {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    println!("couldnt connect to local server");
                    continue;
                }

                let client = { self.client.lock().unwrap().clone() };
                let response = client
                    .get(format!("https://127.0.0.1:{}{}", port, path))
                    .send()
                    .await?;
                let status = response.status();

                if !status.is_success() {
                    self.shared.order_reset(true);
                    if !skip_reset_check {
                        loop {
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

                let bytes = response.bytes().await?;
                break Ok(RiotResponse {
                    bytes: bytes,
                    status: status,
                });
            }
        }
        .await;
        if result.is_err() {
            self.shared.order_reset(true);
        }
        result
    }

    pub async fn get_entitlements_data(
        &self,
        skip_reset_check: bool,
    ) -> Result<Entitlements, RequestError> {
        let res = self
            .send_request("/entitlements/v1/token", skip_reset_check)
            .await?
            .get_json::<Entitlements>()?;
        Ok(res)
    }

    pub fn read_lockfile() -> Result<String, std::io::Error> {
        let local_appdata = env::var("LOCALAPPDATA").expect("LOCALAPPDATA not found");

        let path = format!(r"{}\Riot Games\Riot Client\Config\lockfile", local_appdata);

        fs::read_to_string(&path)
    }
    pub async fn get_gamestate(&self, skip_reset_check: bool) -> Result<String, RequestError> {
        let res = self.get_private_presence(skip_reset_check).await?;
        let match_p = res
            .match_presence_data
            .ok_or(RequestError::Unknown("cannot get match presence".into()))?;
        let session_loop = match_p.session_loop_state.ok_or(RequestError::Unknown(
            "cannot get session loop state".into(),
        ))?;
        Ok(session_loop)
    }
    pub async fn get_account_alias(
        &self,
        skip_reset_check: bool,
    ) -> Result<AccountAlias, RequestError> {
        let res = self
            .send_request("/player-account/aliases/v1/active", skip_reset_check)
            .await?
            .get_json::<AccountAlias>()?;
        Ok(res)
    }
    pub async fn get_external_session(
        &self,
        skip_reset_check: bool,
    ) -> Result<Value, RequestError> {
        let res = self
            .send_request("/product-session/v1/external-sessions", skip_reset_check)
            .await?
            .get_json::<Value>()?;
        Ok(res)
    }
    pub async fn get_presences(&self, skip_reset_check: bool) -> Result<Presence, RequestError> {
        Ok(self
            .send_request("/chat/v4/presences", skip_reset_check)
            .await?
            .get_json::<Presence>()?)
    }
    pub async fn get_my_presence(
        &self,
        skip_reset_check: bool,
    ) -> Result<PresenceData, RequestError> {
        let me = { self.shared.get_user().clone().puuid };
        Ok(self
            .get_presences(skip_reset_check)
            .await?
            .presences
            .iter()
            .find(|wa| wa.puuid == me && wa.product == "valorant")
            .ok_or(RequestError::Unknown("Cannot find me".into()))?
            .clone())
    }
    pub async fn get_private_presence(
        &self,
        skip_reset_check: bool,
    ) -> Result<PrivatePresence, RequestError> {
        let me = self.get_my_presence(skip_reset_check).await?;
        let encoded = me
            .private
            .ok_or(RequestError::Unknown("Cannot get private presence".into()))?;
        let decoded = general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| RequestError::Unknown(e.to_string()))?;
        let private =
            String::from_utf8(decoded).map_err(|e| RequestError::Unknown(e.to_string()))?;
        let presence = serde_json::from_str::<PrivatePresence>(&private)
            .map_err(|e| RequestError::Unknown(e.to_string()))?;
        Ok(presence)
    }
    pub async fn get_region(&self, skip_reset_check: bool) -> Result<String, RequestError> {
        let res = self.get_external_session(skip_reset_check).await?;

        let key = res
            .as_object()
            .ok_or(RequestError::Unknown(
                "Failed to obtain session json".into(),
            ))?
            .keys()
            .find(|k| *k != "host_app")
            .ok_or(RequestError::Unknown("Cannot find key".into()))?
            .to_string();

        let arguments = res[&key]["launchConfiguration"]["arguments"]
            .as_array()
            .ok_or(RequestError::Unknown(
                "Failed to get arguments as array".into(),
            ))?;

        let region = arguments
            .get(4)
            .and_then(|we| we.as_str())
            .ok_or(RequestError::Unknown("Region argument not found".into()))?
            .split("=")
            .nth(1)
            .ok_or(RequestError::Unknown("Failed to get region".into()))?
            .to_string();
        Ok(region)
    }
    pub async fn build(&self) -> Result<(), String> {
        let lockfile = Self::read_lockfile().map_err(|e| {
            format!(
                "[LocalClient::build] I CANT READ LOCKFILE: {}",
                e.to_string()
            )
        })?;

        let split: Vec<&str> = lockfile.split(":").collect();
        let port = split[2];
        let pass = split[3];

        let auth = { self.shared.get_auth().clone() };
        *self.shared.get_auth() = AuthToken {
            lockfile_port: port.into(),
            lockfile_password: pass.into(),
            ..auth
        };

        let basic_authorization = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("riot:{}", pass))
        );

        let mut header = HeaderMap::new();
        header.insert(
            AUTHORIZATION,
            HeaderValue::from_str(basic_authorization.as_str()).unwrap(),
        );
        let new_client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(header)
            .build()
            .unwrap();

        *self.client.lock().unwrap() = new_client;

        self.initialized_http.set_and_notify(true);

        let entitlements = self
            .get_entitlements_data(true)
            .await
            .map_err(|e| format!("Failed to obtain entitlements data: {}", e))?;
        let entitlements_token = entitlements.token;
        let access_token = entitlements.access_token;

        let current_auth = { self.shared.get_auth().clone() };
        *self.shared.get_auth() = AuthToken {
            entitlements_token: entitlements_token,
            access_token: access_token,
            ..current_auth
        };

        let alias = self
            .get_account_alias(true)
            .await
            .map_err(|e| format!("Failed to obtain account alias: {}", e))?;

        let region = loop {
            let region = match self.get_region(true).await {
                Ok(v) => v,
                Err(_) => continue,
            };
            break region;
        };

        *self.shared.get_user() = UserData {
            puuid: entitlements.subject,
            name: alias.game_name,
            tag: alias.tag_line,
            region: region.clone(),
        };

        self.initialized.set_and_notify(true);
        Ok(())
    }
    pub fn new(shared: Arc<SharedGameData>) -> Self {
        Self {
            shared: shared,
            client: Mutex::new(Client::new()),
            initialized: NotifyStruct::default(),
            initialized_http: NotifyStruct::default(),
        }
    }
}
