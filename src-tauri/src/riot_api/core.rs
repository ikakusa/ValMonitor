use super::notify::NotifyStruct;
use crate::riot_api::clients::{
    henrik_client::HenrikClient, local_client::LocalClient, pvp_client::PvPClient,
};
use crate::riot_api::shared_data::SharedGameData;
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::{sync::Arc, time::Duration};

pub struct ValorantAPI {
    pub local_client: Arc<LocalClient>,
    pub pvp_client: Arc<PvPClient>,
    pub henrik_client: Arc<HenrikClient>,
    pub raw_lockfile: Mutex<String>,
    pub initialized: NotifyStruct,
    pub shared: Arc<SharedGameData>,
}

pub static VALORANT_API: Lazy<Arc<ValorantAPI>> = Lazy::new(|| Arc::new(ValorantAPI::new()));

impl ValorantAPI {
    pub fn monitor_lockfile(self: Arc<Self>) {
        tauri::async_runtime::spawn(async move {
            loop {
                match LocalClient::read_lockfile() {
                    Ok(content) => {
                        let mut update_when_req_err = false;
                        let should_update = {
                            if self.shared.should_reset() {
                                update_when_req_err = true;
                                true
                            } else {
                                let mut raw_lock = self.raw_lockfile.lock().unwrap();
                                if content != *raw_lock {
                                    *raw_lock = content.clone();
                                    true
                                } else {
                                    false
                                }
                            }
                        };

                        if self.henrik_client.initialized.value.load(Ordering::Acquire) {
                            let apikey = { self.henrik_client.apikey.lock().unwrap().clone() };
                            let read_apikey = self.henrik_client.read_apikey().await.unwrap();
                            if apikey != read_apikey {
                                let _ = self.henrik_client.build().await;
                                println!("Updated Henrik Client");
                            }
                        }

                        if !should_update {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }

                        match self.local_client.build().await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("{}", err)
                            }
                        }
                        match self.pvp_client.build().await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("{}", err)
                            }
                        }
                        match self.henrik_client.build().await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("{}", err.to_string())
                            }
                        }
                        if !update_when_req_err {
                            println!("[ValorantApi::monitor_lockfile] Updated clients");
                        }
                        self.shared.order_reset(false);
                    }
                    Err(_err) => continue,
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }
    pub fn is_initialized(&self) -> bool {
        self.initialized.value.load(Ordering::Acquire)
    }
    pub fn new() -> Self {
        let dir = ProjectDirs::from("com", "ikakusa", "valmonitor").expect("Couldnt resolve path");
        let app_data_path = dir.data_dir().to_path_buf();
        let mut data = SharedGameData::default();
        data.app_data_path = app_data_path;

        let request_arc = Arc::new(data);

        Self {
            local_client: Arc::new(LocalClient::new(request_arc.clone())),
            pvp_client: Arc::new(PvPClient::new(request_arc.clone())),
            henrik_client: Arc::new(HenrikClient::new(request_arc.clone())),
            initialized: NotifyStruct::default(),
            raw_lockfile: Mutex::new(String::new()),
            shared: request_arc.clone(),
        }
    }
}
