use std::str::FromStr;

use jup_ag::{Quote, QuoteConfig};
use solana_sdk::pubkey::Pubkey;
use eyre::Result;

pub struct Jupiter;

impl Jupiter {
  pub async fn quote(
    input_mint: &str,
    amount: u64,
    slippage_bps: u64,
  ) -> Result<Quote> {
    let sol = Pubkey::from_str("So11111111111111111111111111111111111111112")?;
    let response = jup_ag::quote(
      Pubkey::from_str(input_mint)?,
      sol,
      amount,
      QuoteConfig {
        only_direct_routes: true,
        slippage_bps: Some(slippage_bps),
        ..QuoteConfig::default()
      },
    ).await?;

    Ok(response)
  }
}
