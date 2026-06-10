use super::notify::NotifyStruct;
use crate::riot_api::clients::{local_client::LocalClient, pvp_client::PvPClient};
use crate::riot_api::shared_data::SharedGameData;
use once_cell::sync::Lazy;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::{sync::Arc, time::Duration};

pub struct ValorantAPI {
    pub local_client: Arc<LocalClient>,
    pub pvp_client: Arc<PvPClient>,
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
                        let should_update = {
                            if self.shared.should_reset() {
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
                        println!("[ValorantApi::monitor_lockfile] Updated clients");
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
        let request_arc = Arc::new(SharedGameData::default());

        Self {
            local_client: Arc::new(LocalClient::new(request_arc.clone())),
            pvp_client: Arc::new(PvPClient::new(request_arc.clone())),
            initialized: NotifyStruct::default(),
            raw_lockfile: Mutex::new(String::new()),
            shared: request_arc.clone(),
        }
    }
}
