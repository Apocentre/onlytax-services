use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "TELOXIDE_TOKEN")]
  pub teloxide_token: String,

}
