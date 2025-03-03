use envconfig::Envconfig;
use onlytax_blockchain::helius::helius_api::HeliusApi;
use crate::storage::Storage;

use super::config::Config;

pub struct Store {
  pub config: Config,
  pub storage: Storage,
  pub helius_api: HeliusApi,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::init_from_env().unwrap();
    let storage = Storage::new();
    let helius_api = HeliusApi::new(config.helius_api.clone());

    Self {
      config,
      storage,
      helius_api,
    }
  }
}
