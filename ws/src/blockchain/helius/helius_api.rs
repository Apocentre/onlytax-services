use log::info;
use reqwest::Client;
use eyre::Result;
use super::types::{
  Body, TokenAccountParam, Response, TokenAccountResponse,
  PriorityParam, PriorityOption, PriorityFeeResponse,
};

pub struct HeliusApi {
  pub api: String,
}

impl HeliusApi {
  const PAGINATION_LIMIT: u32 = 1;

  pub fn new(api: String, api_key: String) -> Self {
    let api = format!("{}?api-key={}", api, api_key);
    Self {api}
  }

  pub async fn fetch_token_accounts(&self, mint: &str) -> Result<Vec<TokenAccountResponse>> {
    let mut page = 1;
    let mut response = self.send_fetch_token_accounts(mint, page).await?;
    let mut total = response.total;
    let mut result = vec![response];

    while total > 0 {
      page += 1;
      response = self.send_fetch_token_accounts(mint, page).await?;
      total = response.total;
      result.push(response);
    }
  
    Ok(result)
  }

  pub async fn fetch_priority_fee(&self) -> Result<PriorityFeeResponse> {
    info!("sending getPriorityFeeEstimate");

    let client = Client::new();
    let response = client
    .post(&self.api)
    .json(&Body {
      jsonrpc: "2.0",
      id: 1,
      method: "getPriorityFeeEstimate",
      params: PriorityParam {
        options: PriorityOption {
          include_all_priority_fee_levels: true,
          lookback_slots: 100,
        }
      },
    })
    .send().await?;

    let Response::<PriorityFeeResponse> {result: response} = response.json().await?;

    Ok(response)
  }

  async fn send_fetch_token_accounts(&self, mint: &str, page: u32) -> Result<TokenAccountResponse> {
    info!("sending getTokenAccounts for token {mint} to Helius for page {page}");

    let client = Client::new();
    let response = client
    .post(&self.api)
    .json(&Body {
      jsonrpc: "2.0",
      id: 1,
      method: "getTokenAccounts",
      params: TokenAccountParam {
        mint,
        page,
        limit: Self::PAGINATION_LIMIT,
      },
    })
    .send().await?;

    let Response::<TokenAccountResponse> {result: response} = response.json().await?;

    Ok(response)
  }
}
