use std::{ops::Deref, str::FromStr};
use eyre::Report;
use envconfig::Envconfig;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "PORT")]
  pub port: u64,
  #[envconfig(from = "SOLANA_RPC")]
  pub solana_rpc: String,
  #[envconfig(from = "OPERATOR_PRIV_KEY")]
  pub operator_keypair: SolanaKeypair,
  #[envconfig(from = "OPERATOR_PRIV_KEY")]
  pub treasury: SolanaPubkey,
  #[envconfig(from = "PROTOCOL_FEE_BPS")]
  pub protocol_fee_bps: u64,
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

#[derive(Clone, Copy)]
pub struct SolanaPubkey(Pubkey);

impl FromStr for SolanaPubkey {
  type Err = Report;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let pubkey = Pubkey::from_str(s).expect("valid pubkey");
    Ok(Self(pubkey))
  }
}

impl Deref for SolanaPubkey {
  type Target = Pubkey;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
