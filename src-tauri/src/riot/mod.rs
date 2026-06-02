mod assets;
mod auth;
mod local;
mod presence;
mod pvp;
mod watcher;

use reqwest::Client;
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, MutexGuard};
use tokio::sync::Notify;

use crate::errors::{AppError, AppResult};
use crate::models::{auth::AuthSnapshot, lockfile::LockfileData};

#[derive(Clone)]
pub(super) struct LocalSession {
    pub lockfile: LockfileData,
    pub client: Client,
}

#[derive(Clone)]
pub(super) struct AuthSession {
    pub client: Client,
}

pub struct ValorantApi {
    raw_lockfile: Mutex<String>,
    local_session: Mutex<Option<LocalSession>>,
    auth_session: Mutex<Option<AuthSession>>,
    pvp_client: Mutex<Option<Client>>,
    region: Mutex<Option<String>>,
    auth_snapshot: Mutex<AuthSnapshot>,

    local_initialized: AtomicBool,
    local_initialized_notify: Notify,

    auth_initialized: AtomicBool,
    auth_initialized_notify: Notify,

    pvp_initialized: AtomicBool,
    pvp_initialized_notify: Notify,
}

impl ValorantApi {
    pub fn new() -> Self {
        Self {
            raw_lockfile: Mutex::new(String::new()),
            local_session: Mutex::new(None),
            auth_session: Mutex::new(None),
            pvp_client: Mutex::new(None),
            region: Mutex::new(None),
            auth_snapshot: Mutex::new(AuthSnapshot::default()),
            local_initialized: AtomicBool::new(false),
            local_initialized_notify: Notify::new(),
            auth_initialized: AtomicBool::new(false),
            auth_initialized_notify: Notify::new(),
            pvp_initialized: AtomicBool::new(false),
            pvp_initialized_notify: Notify::new(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.pvp_initialized
            .load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn full_username(&self) -> AppResult<String> {
        Ok(self
            .auth_snapshot
            .lock_app("auth_snapshot")?
            .full_username())
    }

    pub fn puuid(&self) -> AppResult<String> {
        Ok(self.auth_snapshot.lock_app("auth_snapshot")?.puuid.clone())
    }
}

pub(super) trait MutexExt<T> {
    fn lock_app(&self, name: &'static str) -> AppResult<MutexGuard<'_, T>>;
}

impl<T> MutexExt<T> for Mutex<T> {
    fn lock_app(&self, name: &'static str) -> AppResult<MutexGuard<'_, T>> {
        // Mutex poisoning は内部状態が途中で壊れた可能性を示すため、
        // unwrap で process を落とさず command error として UI に返す。
        self.lock().map_err(|_| AppError::StatePoisoned(name))
    }
}
