use crate::riot_api::{
    notify::NotifyStruct,
    response::{RequestError, RiotResponse},
    shared_data::SharedGameData,
};
use reqwest::{header, Client};
use serde_json::Value;
use std::{
    fs,
    sync::{atomic::Ordering, Arc, Mutex},
    time::Duration,
};
use tauri::http::{HeaderMap, HeaderValue};

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
            let client = self.client.lock().unwrap();
            let res = client
                .get(format!("https://api.henrikdev.xyz/valorant{}", path))
                .send()
                .await?;
            let status = res.status();
            if !status.is_success() {
                let _ = self.build().await;
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }

            let bytes = res.bytes().await?;
            break Ok(RiotResponse {
                bytes: bytes,
                status: status,
            });
        }
    }
    pub async fn get_account_by_id(&self, puuid: &str) -> Result<Value, RequestError> {
        Ok(self
            .send_request(format!("/v2/by-puuid/account/{puuid}{}", puuid).as_str())
            .await?
            .get_json::<Value>()?)
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
