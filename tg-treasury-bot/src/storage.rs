use std::sync::atomic::{AtomicBool, Ordering};

pub struct Storage {
  pub enabled: AtomicBool,
}

impl Storage {
  pub fn new() -> Self {
    Self {
      enabled: AtomicBool::new(false),
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
}
