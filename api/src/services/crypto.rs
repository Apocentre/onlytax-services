use std::str::FromStr;
use eyre::Result;
use ed25519_dalek::{PublicKey, Signature, Verifier};

pub fn verify_sig(msg: &[u8], pub_key: &[u8], sig: &str) -> Result<()> {
  let pubkey = bs58::decode(pub_key).into_vec()?;
  let public_key: PublicKey = PublicKey::from_bytes(&pubkey)?;

  Ok(public_key.verify(msg, &Signature::from_str(sig)?)?)
}
