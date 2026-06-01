use base64::{engine::general_purpose, Engine as _};
use reqwest;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Notify;

pub struct ValorantAPI {
    pub raw_lockfile: Mutex<String>,
    pub server_name: Mutex<String>,
    pub server_pid: Mutex<String>,
    pub server_port: Mutex<String>,
    pub server_password: Mutex<String>,
    pub server_protocol: Mutex<String>,
    pub server_header_auth: Mutex<String>,
    pub client: Mutex<Client>,
    pub auth_client: Mutex<Client>,
    pub pvp_client: Mutex<Client>,
    pub access_token: Mutex<String>,
    pub puuid: Mutex<String>,
    pub entitlements_token: Mutex<String>,

    pub name: Mutex<String>,
    pub tag_line: Mutex<String>,

    local_initialized: AtomicBool,
    local_initialized_notify: Notify,

    auth_initialized: AtomicBool,
    auth_initialized_notify: Notify,

    pvp_initialized: AtomicBool,
    pvp_initialized_notify: Notify,
}

impl ValorantAPI {
    pub async fn is_initialized(&self) -> bool {
        return self.pvp_initialized.load(Ordering::Acquire);
    }
    // local endpoint
    pub async fn get_presence(&self) -> Result<Value, String> {
        self.get_local_request("/chat/v4/presences").await
    }

    pub async fn get_my_presence(&self) -> Result<Value, String> {
        while !self.auth_initialized.load(Ordering::Acquire) {
            self.auth_initialized_notify.notified().await;
        }
        match self.get_presence().await {
            Ok(json_obj) => match json_obj["presences"].as_array() {
                Some(v) => {
                    let res = v
                        .iter()
                        .filter(|&value: &&Value| value["puuid"] == *self.puuid.lock().unwrap())
                        .cloned()
                        .collect::<Vec<serde_json::Value>>();

                    if res.len() > 0 {
                        return Ok(json!(res[0]));
                    }
                    return Err(String::from("Invalid Json"));
                }
                None => {
                    return Err(String::from("Invalid Json"));
                }
            },
            Err(_err) => Err(String::from("Invalid Json")),
        }
    }

    pub async fn get_private_presence(&self) -> Result<Value, String> {
        while !self.auth_initialized.load(Ordering::Acquire) {
            self.auth_initialized_notify.notified().await;
        }
        match self.get_my_presence().await {
            Ok(json_obj) => {
                if json_obj["private"].as_str().is_none() {
                    return Err(String::from("Invalid Json"));
                }

                let decoded = String::from_utf8(
                    general_purpose::STANDARD
                        .decode(json_obj["private"].as_str().unwrap())
                        .unwrap(),
                )
                .unwrap();
                let parsed: Value = serde_json::from_str(&decoded).map_err(|e| e.to_string())?;

                Ok(parsed)
            }
            Err(_err) => Err(String::from("Invalid Json")),
        }
    }

    pub async fn get_gamestate(&self) -> Result<String, String> {
        match self.get_private_presence().await {
            Ok(json) => Ok(String::from(
                json["matchPresenceData"]["sessionLoopState"]
                    .as_str()
                    .unwrap(),
            )),
            Err(_err) => Err(String::from("Invalid Json")),
        }
    }

    pub async fn get_entitlements_token(&self) -> Result<Value, String> {
        self.get_local_request("/entitlements/v1/token").await
    }

    async fn get_local_request(&self, path: &str) -> Result<Value, String> {
        while !self.local_initialized.load(Ordering::Acquire) {
            self.local_initialized_notify.notified().await;
        }

        let client = self.client.lock().unwrap().clone();

        match client
            .get(format!(
                "https://127.0.0.1:{}{}",
                *self.server_port.lock().unwrap(),
                path
            ))
            .send()
            .await
        {
            Ok(response) => match response.json::<Value>().await {
                Ok(json) => {
                    // println!("{}", json);
                    Ok(json)
                }
                Err(_error) => Err(String::from("Failed to get json")),
            },
            Err(_error) => Err(String::from("Failed to send request")),
        }
    }

    //

    // auth endpoint
    async fn get_auth_request(&self, path: &str) -> Result<Value, String> {
        while !self.auth_initialized.load(Ordering::Acquire) {
            self.auth_initialized_notify.notified().await;
        }

        let client = self.auth_client.lock().unwrap().clone();

        match client
            .get(format!("https://auth.riotgames.com{}", path))
            .send()
            .await
        {
            Ok(response) => match response.json::<Value>().await {
                Ok(json) => {
                    // println!("{}", json);
                    Ok(json)
                }
                Err(_error) => Err(String::from("Failed to get json")),
            },
            Err(_error) => Err(String::from("Failed to send request")),
        }
    }

    pub async fn get_userinfo(&self) -> Result<Value, String> {
        self.get_auth_request("/userinfo").await
    }
    //

    fn read_lockfile(&self) -> Result<String, std::io::Error> {
        let local_appdata = env::var("LOCALAPPDATA").expect("LOCALAPPDATA not found");

        let path = format!(r"{}\Riot Games\Riot Client\Config\lockfile", local_appdata);

        fs::read_to_string(&path)
    }

    pub fn monitor_lockfile(self: Arc<Self>) {
        tauri::async_runtime::spawn(async move {
            loop {
                match self.read_lockfile() {
                    Ok(content) => {
                        let mut puuid_check = String::new();
                        if self.is_initialized().await {
                            let entitlements_for_check = self.get_entitlements_token().await;
                            match entitlements_for_check {
                                Ok(v) => match v["subject"].as_str() {
                                    Some(v) => {
                                        let subject = v.to_string();
                                        puuid_check = subject.clone();
                                        tokio::time::sleep(Duration::from_secs(1)).await;
                                    }
                                    None => {
                                        tokio::time::sleep(Duration::from_secs(1)).await;
                                    }
                                },
                                Err(_) => {
                                    tokio::time::sleep(Duration::from_secs(1)).await;
                                }
                            }
                        }

                        let split: Vec<&str> = content.split(":").collect();

                        if split.len() < 5 {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }

                        if *self.raw_lockfile.lock().unwrap() == content
                            && *self.puuid.lock().unwrap() == puuid_check
                        {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }

                        *self.raw_lockfile.lock().unwrap() = content.clone();

                        *self.server_name.lock().unwrap() = split[0].to_string();
                        *self.server_pid.lock().unwrap() = split[1].to_string();
                        *self.server_port.lock().unwrap() = split[2].to_string();
                        *self.server_password.lock().unwrap() = split[3].to_string();
                        *self.server_protocol.lock().unwrap() = split[4].to_string();

                        *self.server_header_auth.lock().unwrap() = format!(
                            "Basic {}",
                            general_purpose::STANDARD.encode(format!("riot:{}", split[3]))
                        );

                        let mut default_headers = reqwest::header::HeaderMap::new();
                        default_headers.insert(
                            reqwest::header::AUTHORIZATION,
                            reqwest::header::HeaderValue::from_str(
                                self.server_header_auth.lock().unwrap().as_str(),
                            )
                            .unwrap(),
                        );

                        *self.client.lock().unwrap() = reqwest::Client::builder()
                            .danger_accept_invalid_certs(true)
                            .default_headers(default_headers)
                            .build()
                            .unwrap();

                        self.local_initialized.store(true, Ordering::Release);
                        self.local_initialized_notify.notify_waiters();

                        let entitlements = match self.get_entitlements_token().await {
                            Ok(v) => v,
                            Err(_) => {
                                continue;
                            }
                        };

                        if entitlements["accessToken"].as_str().is_none() {
                            continue;
                        }

                        *self.access_token.lock().unwrap() =
                            entitlements["accessToken"].as_str().unwrap().to_string();

                        *self.entitlements_token.lock().unwrap() =
                            entitlements["token"].as_str().unwrap().to_string();

                        *self.puuid.lock().unwrap() =
                            entitlements["subject"].as_str().unwrap().to_string();

                        let mut auth_headers = reqwest::header::HeaderMap::new();
                        auth_headers.insert(
                            reqwest::header::AUTHORIZATION,
                            reqwest::header::HeaderValue::from_str(
                                format!("Bearer {}", self.access_token.lock().unwrap()).as_str(),
                            )
                            .unwrap(),
                        );

                        *self.auth_client.lock().unwrap() = reqwest::Client::builder()
                            .danger_accept_invalid_certs(true)
                            .default_headers(auth_headers)
                            .build()
                            .unwrap();

                        self.auth_initialized.store(true, Ordering::Release);
                        self.auth_initialized_notify.notify_waiters();

                        let userinfo = match self.get_userinfo().await {
                            Ok(v) => v,
                            Err(_) => {
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                continue;
                            }
                        };

                        *self.name.lock().unwrap() =
                            userinfo["acct"]["game_name"].as_str().unwrap().to_string();

                        *self.tag_line.lock().unwrap() =
                            userinfo["acct"]["tag_line"].as_str().unwrap().to_string();

                        self.pvp_initialized.store(true, Ordering::Release);
                        self.pvp_initialized_notify.notify_waiters();

                        println!("[Riot::Monitor] updated headers");
                    }
                    Err(_) => {
                        println!("lockfileerror")
                    }
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }
    pub fn new() -> Self {
        Self {
            raw_lockfile: Mutex::new(String::new()),
            server_pid: Mutex::new(String::new()),
            server_name: Mutex::new(String::new()),
            server_port: Mutex::new(String::new()),
            server_password: Mutex::new(String::new()),
            server_protocol: Mutex::new(String::new()),
            server_header_auth: Mutex::new(String::new()),
            client: Mutex::new(Client::new()),
            auth_client: Mutex::new(Client::new()),
            pvp_client: Mutex::new(Client::new()),
            local_initialized: AtomicBool::new(false),
            local_initialized_notify: Notify::new(),
            auth_initialized: AtomicBool::new(false),
            auth_initialized_notify: Notify::new(),
            pvp_initialized: AtomicBool::new(false),
            pvp_initialized_notify: Notify::new(),
            entitlements_token: Mutex::new(String::new()),
            access_token: Mutex::new(String::new()),
            puuid: Mutex::new(String::new()),
            name: Mutex::new(String::new()),
            tag_line: Mutex::new(String::new()),
        }
    }
}
