use envconfig::Envconfig;
use super::config::Config;

pub struct Store {
  pub config: Config,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::init_from_env().unwrap();

    Self {config}
  }
}
