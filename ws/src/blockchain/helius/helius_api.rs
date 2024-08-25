use reqwest::Client;
use eyre::Result;
use super::types::{Body, TokenAccountParam, Response, TokenAccountResponse};

pub struct HeliusApi {
  pub api: String,
}

impl HeliusApi {
  const PAGINATION_LIMIT: u32 = 1000;

  pub fn new(api: String, api_key: String) -> Self {
    let api = format!("{}?api-key={}", api, api_key);
    Self {api}
  }

  pub async fn fetch_token_accounts(&self, mint: &str) -> Result<Vec<TokenAccountResponse>> {
    let mut page = 1;
    let mut response = self.send_fetch_token_accounts(mint, page).await?;
    let total = response.total;
    let mut result = vec![response];

    while total >= page * Self::PAGINATION_LIMIT {
      page += 1;
      response = self.send_fetch_token_accounts(mint, page).await?;
      result.push(response);
    }
  
    Ok(result)
  }

  async fn send_fetch_token_accounts(&self, mint: &str, page: u32) -> Result<TokenAccountResponse> {
    let client = Client::new();
  
    let response = client
    .post(&self.api)
    .json(&Body {
      jsonrpc: "2.0",
      id: "helius-test",
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
