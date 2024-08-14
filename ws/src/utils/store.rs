use envconfig::Envconfig;
use solana_client::nonblocking::rpc_client::RpcClient;
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub rpc_client: RpcClient,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::init_from_env().unwrap();
    let rpc_client = RpcClient::new(config.solana_rpc.clone());

    Self {
      config,
      rpc_client,
    }
  }
}
