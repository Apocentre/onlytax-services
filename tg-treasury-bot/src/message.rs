use jup_ag::Quote;
use solana_sdk::native_token::lamports_to_sol;

const CHUNK_SIZE: usize = 10;

pub fn format_message(quotes: Vec<Quote>) -> Vec<String> {
  let chunks = quotes.chunks(CHUNK_SIZE);
  let mut messages = Vec::with_capacity(quotes.len().div_ceil(CHUNK_SIZE));

  for chunk in chunks {
    let mut rows = Vec::with_capacity(quotes.len());

    for quote in chunk {
      let token = format!("[Explorer](https://solscan.io/token/{})", quote.input_mint);
      let sol_amount = lamports_to_sol(quote.out_amount).to_string().replace(".", "\\.");
      let raydium_link = format!("[Swap](https://raydium.io/swap/?inputMint={}&outputMint=sol)", quote.input_mint);

      let row = format!(
      r#"
      {} {} {}

      "#,
      token, sol_amount, raydium_link
      );

      rows.push(row);
    }

    let message = rows.into_iter().reduce(|body, row| body + &row).unwrap();
    messages.push(message);
  }

  messages
}
