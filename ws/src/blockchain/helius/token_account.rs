use reqwest::Client;
use eyre::Result;
use super::types::{Body, TokenAccountParam};

pub struct HeliusApi {
  pub api: String,
}

impl HeliusApi {
  pub fn new(api: String, api_key: String) -> Self {
    let api = format!("{}?api-key={}", api, api_key);
    Self {api}
  }

  pub async fn fetch_token_accounts(&self, mint: &str) -> Result<()> {
    let client = Client::new();
  
    let response = client
    .post(&self.api)
    .json(&Body {
      jsonrpc: "2.0",
      id: "helius-test",
      method: "getTokenAccounts",
      params: TokenAccountParam {
        mint,
        page: 1,
        limit: 1,
      },
    })
    .send().await?;

    println!("{}", response.text().await?);
  
    todo!()
  }
}
