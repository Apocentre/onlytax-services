use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "TELOXIDE_TOKEN")]
  pub teloxide_token: String,
  #[envconfig(from = "HELIUS_API")]
  pub helius_api: String,
  #[envconfig(from = "POLL_INTERVAL_SECS")]
  pub poll_interval_secs: u64,
  #[envconfig(from = "TREASURY")]
  pub treasury: String,
}
