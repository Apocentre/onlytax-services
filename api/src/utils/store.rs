use std::sync::Arc;
use onlytax_blockchain::helius::helius_api::HeliusApi;
use onlytax_storage::connection_pool::ConnectionPool;
use envconfig::Envconfig;
use crate::services::auth::Auth;
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub pg_pool: ConnectionPool,
  pub auth: Arc<Auth>,
  pub helius_api: Arc<HeliusApi>,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::init_from_env().unwrap();
    let pg_pool = ConnectionPool::new(&config.postgres_uri).await;
    let auth = Arc::new(Auth::new(&config.jwt_hmac_key));
    let helius_api = Arc::new(HeliusApi::new(config.helius_api.clone()));

    Self {
      config,
      pg_pool,
      auth,
      helius_api,
    }
  }
}
