use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

pub struct Storage {
  pub enabled: AtomicBool,
  pub chat_id: AtomicI64,
}

impl Storage {
  pub fn new() -> Self {
    Self {
      enabled: AtomicBool::new(false),
      chat_id: AtomicI64::new(0),
    }
  }

  pub fn enable(&self) {
    self.enabled.store(true, Ordering::Relaxed);
  }

  pub fn disable(&self) {
    self.enabled.store(false, Ordering::Relaxed);
  }

  pub fn enabled(&self) -> bool {
    self.enabled.load(Ordering::Relaxed)
  }

  pub fn set_chat_id(&self, chat_id: i64) {
    self.chat_id.store(chat_id, Ordering::Relaxed);
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id.load(Ordering::Relaxed)
  }
}
