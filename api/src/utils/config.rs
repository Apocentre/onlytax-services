use std::str::FromStr;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "PORT")]
  pub port: Option<u64>,
  #[envconfig(from = "POSTGRES_URI")]
  pub postgres_uri: String,
  #[envconfig(from = "CORS_ORIGIN")]
  pub cors_config: CorsConfig,
  #[envconfig(from = "JWT_HMAC_KEY")]
  pub jwt_hmac_key: String,
}

pub struct CorsConfig {
  pub origin: Vec<String>,
}

impl FromStr for CorsConfig {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      origin: s.split(",").map(|val| val.to_owned()).collect(),
    })
  }
}
