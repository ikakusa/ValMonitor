use super::local_models::{AccountAlias, Entitlements};
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
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::time::Duration;

pub struct LocalClient {
    client: Mutex<Client>,
    initialized: NotifyStruct,
    initialized_http: NotifyStruct,
    shared: Arc<SharedGameData>,
}

impl LocalClient {
    pub async fn send_request(&self, path: &str) -> Result<RiotResponse, RequestError> {
        let result = async {
            while !self.initialized_http.value.load(Ordering::Acquire) {
                self.initialized_http.notifier.notified().await;
            }

            while self.shared.should_reset() {
                self.shared.need_reset.notifier.notified().await;
            }

            loop {
                let port = {
                    let auth = self.shared.get_auth();
                    auth.lockfile_port.clone()
                };

                let client = { self.client.lock().unwrap().clone() };
                let response = client
                    .get(format!("https://127.0.0.1:{}{}", port, path))
                    .send()
                    .await?;
                let status = response.status();

                if !status.is_success() {
                    self.shared.order_reset(true);
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
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

    pub async fn get_entitlements_data(&self) -> Result<Entitlements, RequestError> {
        let res = self
            .send_request("/entitlements/v1/token")
            .await?
            .get_json::<Entitlements>()?;
        Ok(res)
    }

    pub fn read_lockfile() -> Result<String, std::io::Error> {
        let local_appdata = env::var("LOCALAPPDATA").expect("LOCALAPPDATA not found");

        let path = format!(r"{}\Riot Games\Riot Client\Config\lockfile", local_appdata);

        fs::read_to_string(&path)
    }
    pub async fn get_account_alias(&self) -> Result<AccountAlias, RequestError> {
        let res = self
            .send_request("/player-account/aliases/v1/active")
            .await?
            .get_json::<AccountAlias>()?;
        Ok(res)
    }
    pub async fn get_external_session(&self) -> Result<Value, RequestError> {
        let res = self
            .send_request("/product-session/v1/external-sessions")
            .await?
            .get_json::<Value>()?;
        Ok(res)
    }
    pub async fn get_region(&self) -> Result<String, RequestError> {
        let res = self.get_external_session().await?;

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
            .get_entitlements_data()
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
            .get_account_alias()
            .await
            .map_err(|e| format!("Failed to obtain account alias: {}", e))?;

        let region = self
            .get_region()
            .await
            .map_err(|e| format!("Failed to fetch region: {}", e))?;

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
