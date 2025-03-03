use jup_ag::Quote;

const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;
const CHUNK_SIZE: usize = 25;

pub fn lamports_to_sol(lamports: u64) -> f64 {
  lamports as f64 / LAMPORTS_PER_SOL
}

pub fn format_message(quotes: Vec<Quote>) -> Vec<String> {
  let chunks = quotes.chunks(CHUNK_SIZE);
  let mut messages = Vec::with_capacity(quotes.len().div_ceil(CHUNK_SIZE));

  for chunk in chunks {
    let mut rows = Vec::with_capacity(quotes.len());

    for (i, quote) in chunk.iter().enumerate() {
      let token = format!("[🔗](https://solscan.io/token/{})", quote.input_mint);
      let sol_amount = lamports_to_sol(quote.out_amount);
      let raydium_link = format!("[🔗](https://raydium.io/swap/?inputMint={:?}&outputMint=sol)", token);

      let row = format!(
      r#"ª
      {}. {} 
        - {}
        - {}

      "#,
      i, token, sol_amount, raydium_link
      );

      rows.push(row);
    }

    let message = rows.into_iter().reduce(|body, row| body + &row + &"\n").unwrap();
    messages.push(message);
  }

  messages
}
