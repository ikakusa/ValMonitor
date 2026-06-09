// use base64::{engine::general_purpose, Engine as _};
// use reqwest;
// use reqwest::Client;
// use serde_json::json;
// use serde_json::Value;
// use std::env;
// use std::fs;
// use std::sync::atomic::AtomicBool;
// use std::sync::atomic::Ordering;
// use std::sync::Arc;
// use std::sync::Mutex;
// use std::time::Duration;
// use tokio::sync::Notify;
// use crate::riot_api::asset_client::client::AssetClient;
// use crate::riot_api::asset_client::models::PlayerCard;
// use crate::riot_api::response::RequestError;
// use once_cell::sync::Lazy;
// use crate::riot_api::local_client::client::LocalClient;
// pub static VALORANT_API: Lazy<Arc<ValorantAPI>> = Lazy::new(|| Arc::new(ValorantAPI::new()));

// pub struct ValorantAPI {
//     pub local_client: Arc<LocalClient>,

//     pub raw_lockfile: Mutex<String>,
//     pub server_port: Mutex<String>,
//     pub server_password: Mutex<String>,
//     pub server_header_auth: Mutex<String>,
//     pub auth_client: Mutex<Client>,
//     pub pvp_client: Mutex<Client>,
//     access_token: Mutex<String>,
//     pub puuid: Mutex<String>,
//     pub entitlements_token: Mutex<String>,

//     pub name: Mutex<String>,
//     pub tag_line: Mutex<String>,

//     local_initialized: AtomicBool,
//     local_initialized_notify: Notify,

//     auth_initialized: AtomicBool,
//     auth_initialized_notify: Notify,

//     pvp_initialized: AtomicBool,
//     pvp_initialized_notify: Notify,
// }

// impl ValorantAPI {
// pub async fn get_playercard_by_id(&self, id: &str) -> Result<PlayerCard, RequestError> {
//     AssetClient::get_player_card(id).await
// }

// pub async fn is_initialized(&self) -> bool {
//     return self.pvp_initialized.load(Ordering::Acquire);
// }

// // pvp endpoint
// pub async fn get_mmr(&self, puuid: &str) -> Result<Value, String> {
//     self.get_pvp_request(format!("/mmr/v1/players/{puuid}").as_str())
//         .await
// }
// pub async fn get_pvp_request(&self, path: &str) -> Result<Value, String> {
//     while !self.pvp_initialized.load(Ordering::Acquire) {
//         self.pvp_initialized_notify.notified().await;
//     }

//     loop {
//         let client = self.pvp_client.lock().unwrap().clone();
//         let region = self.get_region().await?;

//         match client
//             .get(format!("https://pd.{}.a.pvp.net{}", region, path))
//             .send()
//             .await
//         {
//             Ok(response) => {
//                 let status = response.status();
//                 match response.json::<Value>().await {
//                     Ok(json) => {
//                         if !status.is_success() {
//                             let _ = self.build_access_tokens().await;
//                             self.build_pvp_client().await;
//                             println!("[ValorantAPI::get_pvp_request] request failed with {} status code: {}\ntrying to build new reqwest client", status.as_u16(), path);
//                             tokio::time::sleep(Duration::from_secs(1)).await;
//                             continue;
//                         }
//                         return Ok(json);
//                     }
//                     Err(_) => {
//                         let _ = self.build_access_tokens().await;
//                         self.build_pvp_client().await;
//                         tokio::time::sleep(Duration::from_secs(3)).await;
//                         continue;
//                     }
//                 }
//             }
//             Err(_) => return Err("Failed to send request".to_string()),
//         }
//     }
// }
// //

// // local endpoint
// pub async fn get_presence(&self) -> Result<Value, String> {
//     self.get_local_request("/chat/v4/presences").await
// }

// pub async fn get_region(&self) -> Result<String, String> {
//     let json = self
//         .get_local_request("/product-session/v1/external-sessions")
//         .await?;

//     let region_key = json
//         .as_object()
//         .ok_or("Invalid response")?
//         .keys()
//         .find(|key| *key != "host_app")
//         .ok_or("Region key not found")?
//         .to_string();

//     let arguments = json[&region_key]["launchConfiguration"]["arguments"]
//         .as_array()
//         .ok_or("Arguments not found")?;

//     let arg = arguments
//         .get(4)
//         .and_then(|v| v.as_str())
//         .ok_or("Region argument not found")?;

//     let region = arg.split('=').nth(1).ok_or("Invalid region format")?;

//     Ok(region.to_string())
// }

// pub async fn get_my_presence(&self) -> Result<Value, String> {
//     while !self.auth_initialized.load(Ordering::Acquire) {
//         self.auth_initialized_notify.notified().await;
//     }
//     match self.get_presence().await {
//         Ok(json_obj) => match json_obj["presences"].as_array() {
//             Some(v) => {
//                 let res = v
//                     .iter()
//                     .filter(|&value: &&Value| value["puuid"] == *self.puuid.lock().unwrap())
//                     .cloned()
//                     .collect::<Vec<serde_json::Value>>();

//                 if res.len() > 0 {
//                     return Ok(json!(res[0]));
//                 }
//                 return Err(String::from("Invalid Json"));
//             }
//             None => {
//                 return Err(String::from("Invalid Json"));
//             }
//         },
//         Err(_err) => Err(String::from("Invalid Json")),
//     }
// }

// pub async fn get_private_presence(&self) -> Result<Value, String> {
//     while !self.auth_initialized.load(Ordering::Acquire) {
//         self.auth_initialized_notify.notified().await;
//     }
//     match self.get_my_presence().await {
//         Ok(json_obj) => {
//             if json_obj["private"].as_str().is_none() {
//                 return Err(String::from("Invalid Json"));
//             }

//             let decoded = String::from_utf8(
//                 general_purpose::STANDARD
//                     .decode(json_obj["private"].as_str().unwrap())
//                     .unwrap(),
//             )
//             .unwrap();
//             let parsed: Value = serde_json::from_str(&decoded).map_err(|e| e.to_string())?;

//             Ok(parsed)
//         }
//         Err(_err) => Err(String::from("Invalid Json")),
//     }
// }

// pub async fn get_gamestate(&self) -> Result<String, String> {
//     match self.get_private_presence().await {
//         Ok(json) => Ok(String::from(
//             json["matchPresenceData"]["sessionLoopState"]
//                 .as_str()
//                 .unwrap(),
//         )),
//         Err(_err) => Ok(String::from("IDLE")),
//     }
// }

// pub async fn get_entitlements_token(&self) -> Result<String, > {
//     self.local_client.get_entitlements_token.await;
// }

//

//     // auth endpoint
//     async fn get_auth_request(&self, path: &str) -> Result<Value, String> {
//         while !self.auth_initialized.load(Ordering::Acquire) {
//             self.auth_initialized_notify.notified().await;
//         }

//         let client = self.auth_client.lock().unwrap().clone();

//         match client
//             .get(format!("https://auth.riotgames.com{}", path))
//             .send()
//             .await
//         {
//             Ok(response) => match response.json::<Value>().await {
//                 Ok(json) => {
//                     // println!("{}", json);
//                     Ok(json)
//                 }
//                 Err(_error) => Err(String::from("Failed to get json")),
//             },
//             Err(_error) => Err(String::from("Failed to send request")),
//         }
//     }

//     pub async fn get_userinfo(&self) -> Result<Value, String> {
//         self.get_auth_request("/userinfo").await
//     }
//     //

//     fn read_lockfile(&self) -> Result<String, std::io::Error> {
//         let local_appdata = env::var("LOCALAPPDATA").expect("LOCALAPPDATA not found");

//         let path = format!(r"{}\Riot Games\Riot Client\Config\lockfile", local_appdata);

//         fs::read_to_string(&path)
//     }

//     async fn build_pvp_client(&self) {
//         let mut pvp_headers = reqwest::header::HeaderMap::new();
//         pvp_headers.insert(
//             reqwest::header::AUTHORIZATION,
//             reqwest::header::HeaderValue::from_str(
//                 format!("Bearer {}", self.access_token.lock().unwrap()).as_str(),
//             )
//             .unwrap(),
//         );
//         pvp_headers.insert(
//             "X-Riot-Entitlements-JWT",
//             reqwest::header::HeaderValue::from_str(
//                 self.entitlements_token.lock().unwrap().as_str(),
//             )
//             .unwrap(),
//         );

//         pvp_headers.insert(
//                             "X-Riot-ClientPlatform",
//                             reqwest::header::HeaderValue::from_str(
//                                 "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9",
//                             )
//                             .unwrap(),
//                         );

//         let client_v: String;

//         match AssetClient::get_version().await {
//             Ok(res) => {
//                 client_v = res.data.riot_client_version.as_str().to_string();
//             },
//             Err(err) => {
//                 println!("[ValorantAPI::build_pvp_client] error with: {}", err);
//                 return;
//             }
//         }
//         pvp_headers.insert(
//             "X-Riot-ClientVersion",
//             reqwest::header::HeaderValue::from_str(client_v.as_str()).unwrap(),
//         );

//         *self.pvp_client.lock().unwrap() = reqwest::Client::builder()
//             .danger_accept_invalid_certs(true)
//             .default_headers(pvp_headers)
//             .build()
//             .unwrap();

//         self.pvp_initialized.store(true, Ordering::Release);
//         self.pvp_initialized_notify.notify_waiters();
//     }

//     async fn build_access_tokens(&self) -> Result<(), String> {
//         let entitlements = self.get_entitlements_token().await?;

//         if entitlements["accessToken"].as_str().is_none() {
//             return Err("[ValorantApi::build_access_tokens] accessToken not found".to_string());
//         }

//         *self.access_token.lock().unwrap() =
//             entitlements["accessToken"].as_str().unwrap().to_string();

//         *self.entitlements_token.lock().unwrap() =
//             entitlements["token"].as_str().unwrap().to_string();

//         *self.puuid.lock().unwrap() = entitlements["subject"].as_str().unwrap().to_string();
//         Ok(())
//     }

//     pub fn monitor_lockfile(self: Arc<Self>) {
//         tauri::async_runtime::spawn(async move {
//             loop {
//                 match self.read_lockfile() {
//                     Ok(content) => {
//                         let mut puuid_check = String::new();
//                         if self.is_initialized().await {
//                             let entitlements_for_check = self.get_entitlements_token().await;
//                             match entitlements_for_check {
//                                 Ok(v) => match v["subject"].as_str() {
//                                     Some(v) => {
//                                         let subject = v.to_string();
//                                         puuid_check = subject.clone();
//                                         tokio::time::sleep(Duration::from_secs(1)).await;
//                                     }
//                                     None => {
//                                         tokio::time::sleep(Duration::from_secs(1)).await;
//                                     }
//                                 },
//                                 Err(_) => {
//                                     tokio::time::sleep(Duration::from_secs(1)).await;
//                                 }
//                             }
//                         }

//                         let split: Vec<&str> = content.split(":").collect();

//                         if split.len() < 5 {
//                             tokio::time::sleep(Duration::from_secs(1)).await;
//                             continue;
//                         }

//                         if *self.raw_lockfile.lock().unwrap() == content
//                             && *self.puuid.lock().unwrap() == puuid_check
//                         {
//                             tokio::time::sleep(Duration::from_secs(1)).await;
//                             continue;
//                         }

//                         *self.raw_lockfile.lock().unwrap() = content.clone();

//                         *self.server_port.lock().unwrap() = split[2].to_string();
//                         *self.server_password.lock().unwrap() = split[3].to_string();

//                         *self.server_header_auth.lock().unwrap() = format!(
//                             "Basic {}",
//                             general_purpose::STANDARD.encode(format!("riot:{}", split[3]))
//                         );
//                         let local_client = self.local_client.clone();
//                         let _ = local_client.build().await;

//                         self.local_initialized.store(true, Ordering::Release);
//                         self.local_initialized_notify.notify_waiters();

//                         match self.build_access_tokens().await {
//                             Ok(v) => v,
//                             Err(_err) => {
//                                 continue;
//                             }
//                         }

//                         let mut auth_headers = reqwest::header::HeaderMap::new();
//                         auth_headers.insert(
//                             reqwest::header::AUTHORIZATION,
//                             reqwest::header::HeaderValue::from_str(
//                                 format!("Bearer {}", self.access_token.lock().unwrap()).as_str(),
//                             )
//                             .unwrap(),
//                         );

//                         *self.auth_client.lock().unwrap() = reqwest::Client::builder()
//                             .danger_accept_invalid_certs(true)
//                             .default_headers(auth_headers)
//                             .build()
//                             .unwrap();

//                         self.auth_initialized.store(true, Ordering::Release);
//                         self.auth_initialized_notify.notify_waiters();

//                         let userinfo = match self.get_userinfo().await {
//                             Ok(v) => v,
//                             Err(_) => {
//                                 tokio::time::sleep(Duration::from_secs(1)).await;
//                                 continue;
//                             }
//                         };

//                         *self.name.lock().unwrap() =
//                             userinfo["acct"]["game_name"].as_str().unwrap().to_string();

//                         *self.tag_line.lock().unwrap() =
//                             userinfo["acct"]["tag_line"].as_str().unwrap().to_string();

//                         self.build_pvp_client().await;

//                         println!("[Riot::Monitor] updated headers");
//                     }
//                     Err(_) => {
//                         println!("lockfileerror")
//                     }
//                 }

//                 tokio::time::sleep(Duration::from_secs(1)).await;
//             }
//         });
//     }
//     pub fn new() -> Self {
//         Self {
//             local_client: Arc::new(LocalClient::new()),

//             raw_lockfile: Mutex::new(String::new()),
//             server_port: Mutex::new(String::new()),
//             server_password: Mutex::new(String::new()),
//             server_header_auth: Mutex::new(String::new()),
//             client: Mutex::new(Client::new()),
//             auth_client: Mutex::new(Client::new()),
//             pvp_client: Mutex::new(Client::new()),
//             local_initialized: AtomicBool::new(false),
//             local_initialized_notify: Notify::new(),
//             auth_initialized: AtomicBool::new(false),
//             auth_initialized_notify: Notify::new(),
//             pvp_initialized: AtomicBool::new(false),
//             pvp_initialized_notify: Notify::new(),
//             entitlements_token: Mutex::new(String::new()),
//             access_token: Mutex::new(String::new()),
//             puuid: Mutex::new(String::new()),
//             name: Mutex::new(String::new()),
//             tag_line: Mutex::new(String::new()),
//         }
//     }
// }
