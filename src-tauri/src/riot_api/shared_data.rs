use super::data::{auth_token::AuthToken, user_data::UserData};
use crate::riot_api::notify::NotifyStruct;
use std::{
    path::PathBuf,
    sync::{atomic::Ordering, Mutex, MutexGuard},
};

#[derive(Default)]
pub struct SharedGameData {
    pub user_data: Mutex<UserData>,
    pub auth_data: Mutex<AuthToken>,
    pub need_reset: NotifyStruct,
    pub app_data_path: PathBuf,
}

impl SharedGameData {
    pub fn get_auth(&self) -> MutexGuard<'_, AuthToken> {
        self.auth_data.lock().unwrap()
    }
    pub fn get_user(&self) -> MutexGuard<'_, UserData> {
        self.user_data.lock().unwrap()
    }
    pub fn should_reset(&self) -> bool {
        self.need_reset.value.load(Ordering::Acquire)
    }
    pub fn order_reset(&self, state: bool) {
        self.need_reset.set_and_notify(state);
    }
    pub fn get_region(&self) -> String {
        self.get_user().clone().region
    }
}
