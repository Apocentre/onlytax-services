use std::sync::Arc;
use envconfig::Envconfig;
use solana_client::nonblocking::rpc_client::RpcClient;
use onlytax_api::services::auth::Auth;
use onlytax_storage::connection_pool::ConnectionPool;
use crate::blockchain::{fee_collector::FeeCollector, token_account::HeliusApi};

use super::config::Config;

pub struct Store {
  pub config: Config,
  pub rpc_client: Arc<RpcClient>,
  pub helius_api: Arc<HeliusApi>,
  pub fee_collector: Arc<FeeCollector>,
  pub pg_pool: ConnectionPool,
  pub auth: Arc<Auth>,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::init_from_env().unwrap();
    let rpc_client = Arc::new(RpcClient::new(config.solana_rpc.clone()));
    let helius_api = Arc::new(HeliusApi::new(config.helius.api.clone(), config.helius.api_key.clone()));
    let fee_collector = Arc::new(FeeCollector::new(
      Arc::clone(&rpc_client),
      config.operator_keypair.clone(),
      config.treasury,
      config.protocol_fee_bps,
      config.priority_fee_rpc.clone(),
    ));

    let pg_pool = ConnectionPool::new(&config.postgres_uri).await;
    let auth = Arc::new(Auth::new(&config.jwt_hmac_key));

    Self {
      config,
      rpc_client,
      helius_api,
      fee_collector,
      pg_pool,
      auth,
    }
  }
}
