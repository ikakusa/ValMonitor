use std::sync::Arc;

use crate::discord_rpc::DiscordRpcService;
use crate::henrik::HenrikClient;
use crate::riot::ValorantApi;

pub struct AppState {
    pub riot: Arc<ValorantApi>,
    pub henrik: HenrikClient,
    pub discord_rpc: DiscordRpcService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            riot: Arc::new(ValorantApi::new()),
            henrik: HenrikClient::new(),
            discord_rpc: DiscordRpcService::new(),
        }
    }

    pub fn start_background_tasks(&self) {
        // lockfile 監視はアプリ全体で 1 本だけ動かす必要があるため、
        // Tauri の managed state 初期化直後に起動して command 層からは起動させない。
        self.riot.clone().monitor_lockfile();
    }
}
