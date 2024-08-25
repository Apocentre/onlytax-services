
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Body<'a> {
  pub jsonrpc: &'a str,
  pub id: &'a str,
  pub method: &'a str,
  pub params: TokenAccountParam<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAccountParam<'a> {
  pub mint: &'a str,
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
  pub owner: String,
  pub amount: u64,
  pub token_extensions: TokenExtensions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenExtensions {
  transfer_fee_amount: TransferFeeAmount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFeeAmount {
  withheld_amount: u64,
}
