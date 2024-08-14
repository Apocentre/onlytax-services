use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "PORT")]
  pub port: u64,
  #[envconfig(from = "SOLANA_RPC")]
  pub solana_rpc: String,
}
