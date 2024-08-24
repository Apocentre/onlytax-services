use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_sdk::{compute_budget::ComputeBudgetInstruction, instruction::Instruction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body<'a> {
  pub jsonrpc: &'a str,
  pub id: u8,
  pub method: &'a str,
  pub params: Param,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Param {
  pub last_n_blocks: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
  pub result: Result,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
  pub per_compute_unit: PerComputeUnit,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerComputeUnit {
  pub extreme: u64,
  pub high: u64,
  pub medium: u64,
}

const DEFAULT_PRIORITY_FEE: u64 = 50_000;


/// There are 10^6 micro-lamports in one lamport. 2_500_000 micro lamport is => 2_500_000 / 1_000_000 = 2.5 Lamports
/// The total fees will be: fees = compute budget * U = 200,000 * 2.5 = 500,000 lamport or 0.0005 SOL.
/// This is 100 higher than the default fee which is 0.000005 SOL
pub async fn create_priority_fee_ix(create_priority_fee_ix: &str) -> Instruction {
  let client = Client::new();

  let response = client
  .post(create_priority_fee_ix)
  .json(&Body {
    jsonrpc: "2.0",
    id: 1,
    method: "qn_estimatePriorityFees",
    params: Param {last_n_blocks: 100},
  })
  .send().await;

  if let Err(err) = response {
    error!("Failed to fetch priority fee {}", err);
    return ComputeBudgetInstruction::set_compute_unit_price(DEFAULT_PRIORITY_FEE)
  }

  let response = response.unwrap().json::<Response>().await;
  if let Err(err) = response {
    error!("Failed parsing response from priority fee {} service", err);
    return ComputeBudgetInstruction::set_compute_unit_price(DEFAULT_PRIORITY_FEE)
  }

  let priority_fee = response.unwrap().result.per_compute_unit.extreme;
  ComputeBudgetInstruction::set_compute_unit_price(priority_fee)
}

