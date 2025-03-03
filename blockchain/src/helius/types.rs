
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Body<'a, P> {
  pub jsonrpc: &'a str,
  pub id: u8,
  pub method: &'a str,
  pub params: P,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAccountParam<'a> {
  pub mint: Option<&'a str>,
  pub owner: Option<&'a str>,
  pub page: u32,
  pub limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
  pub result: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAccountResponse {
  pub total: u32,
  pub limit: u32,
  pub page: u32,
  pub token_accounts: Vec<TokenAccount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAccount {
  pub address: String,
  pub mint: String,
  pub owner: String,
  pub amount: u64,
  pub token_extensions: Option<TokenExtensions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenExtensions {
  pub transfer_fee_amount: TransferFeeAmount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFeeAmount {
 pub withheld_amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriorityParam {
  pub options: PriorityOption,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriorityOption {
  pub include_all_priority_fee_levels: bool,
  pub lookback_slots: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriorityFeeResponse {
  pub priority_fee_levels: PriorityFeeLevel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriorityFeeLevel {
  pub min: f64,
  pub low: f64,
  pub medium: f64,
  pub high: f64,
  pub very_high: f64,
  pub unsafe_max: f64,
}
