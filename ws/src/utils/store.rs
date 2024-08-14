use std::sync::Arc;
use envconfig::Envconfig;
use solana_client::nonblocking::rpc_client::RpcClient;
use crate::blockchain::fee_collector::FeeCollector;

use super::config::Config;

pub struct Store {
  pub config: Config,
  pub rpc_client: Arc<RpcClient>,
  pub fee_collector: Arc<FeeCollector>,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::init_from_env().unwrap();
    let rpc_client = Arc::new(RpcClient::new(config.solana_rpc.clone()));
    let fee_collector = Arc::new(FeeCollector::new(Arc::clone(&rpc_client)));

    Self {
      config,
      rpc_client,
      fee_collector,
    }
  }
}
