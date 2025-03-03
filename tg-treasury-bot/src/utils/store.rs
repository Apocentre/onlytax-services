use envconfig::Envconfig;
use onlytax_blockchain::helius::helius_api::HeliusApi;
use crate::storage::Storage;

use super::config::Config;

pub struct Store {
  pub config: Config,
  pub helius_api: HeliusApi,
  pub storage: Storage,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::init_from_env().unwrap();
    let helius_api = HeliusApi::new(config.helius_api.clone());
    let storage = Storage::new();

    Self {
      config,
      helius_api,
      storage,
    }
  }
}
