use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Notify;

#[derive(Default)]
pub struct NotifyStruct {
    pub value: AtomicBool,
    pub notifier: Notify,
}

impl NotifyStruct {
    pub fn set_and_notify(&self, value: bool) {
        self.value.store(value, Ordering::Release);
        self.notifier.notify_waiters();
    }
}
