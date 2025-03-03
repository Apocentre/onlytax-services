use log::info;
use reqwest::Client;
use eyre::Result;
use super::types::{
  Body, PriorityFeeResponse, PriorityOption, PriorityParam, Response, TokenAccount, TokenAccountParam, TokenAccountResponse
};

pub struct HeliusApi {
  pub api: String,
}

impl HeliusApi {
  const PAGINATION_LIMIT: u32 = 1_000;

  pub fn new(api: String) -> Self {
    Self {api}
  }

  pub async fn fetch_token_accounts(&self, mint: &str) -> Result<Vec<TokenAccount>> {
    let mut page = 1;
    let mut response = self.send_fetch_token_accounts(mint, page).await?;
    let total = response.total;
    let mut result = response.token_accounts;

    while total > page * Self::PAGINATION_LIMIT {
      page += 1;
      response = self.send_fetch_token_accounts(mint, page).await?;
      result.extend(response.token_accounts);
    }
  
    Ok(result)
  }

  pub async fn fetch_token_accounts_by_owner(&self, owner: &str) -> Result<Vec<TokenAccount>> {
    let mut page = 1;
    let mut response = self.send_fetch_token_accounts_by_owner(owner, page).await?;
    let total = response.total;
    let mut result = response.token_accounts;

    while total > page * Self::PAGINATION_LIMIT {
      page += 1;
      response = self.send_fetch_token_accounts_by_owner(owner, page).await?;
      result.extend(response.token_accounts);
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
      params: vec![PriorityParam {
        options: PriorityOption {
          include_all_priority_fee_levels: true,
          lookback_slots: 100,
        }
      }],
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
        mint: Some(mint),
        owner: None,
        page,
        limit: Self::PAGINATION_LIMIT,
      },
    })
    .send().await?;

    let Response::<TokenAccountResponse> {result: response} = response.json().await?;

    Ok(response)
  }

  async fn send_fetch_token_accounts_by_owner(&self, owner: &str, page: u32) -> Result<TokenAccountResponse> {
    info!("sending getTokenAccounts owned by {owner} to Helius for page {page}");

    let client = Client::new();
    let response = client
    .post(&self.api)
    .json(&Body {
      jsonrpc: "2.0",
      id: 1,
      method: "getTokenAccounts",
      params: TokenAccountParam {
        mint: None,
        owner: Some(owner),
        page,
        limit: Self::PAGINATION_LIMIT,
      }
    })
    .send().await?;

    let Response::<TokenAccountResponse> {result: response} = response.json().await?;

    Ok(response)
  }
}
