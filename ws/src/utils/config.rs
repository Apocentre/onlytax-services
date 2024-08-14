use std::{ops::Deref, str::FromStr};
use eyre::Report;
use envconfig::Envconfig;
use solana_sdk::signature::Keypair;

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "PORT")]
  pub port: u64,
  #[envconfig(from = "SOLANA_RPC")]
  pub solana_rpc: String,
  #[envconfig(from = "OPERATOR_PRIV_KEY")]
  pub operator_keypair: SolanaKeypair,
}

pub struct SolanaKeypair(Keypair);

impl Clone for SolanaKeypair {
  fn clone(&self) -> Self {
    let clone = self.0.to_base58_string();
    Self(Keypair::from_base58_string(&clone))
  }
}

impl FromStr for SolanaKeypair {
  type Err = Report;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let keypair = Keypair::from_base58_string(s);
    Ok(Self(keypair))
  }
}

impl Deref for SolanaKeypair {
  type Target = Keypair;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
