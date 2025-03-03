use comrak::{markdown_to_html, Options};
use jup_ag::Quote;

const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

pub fn lamports_to_sol(lamports: u64) -> f64 {
  lamports as f64 / LAMPORTS_PER_SOL
}

pub fn format_message(quotes: Vec<Quote>) -> String {
  let header = format!(
    r#"
    |Token|Amount|USD Value|Raydium|
    |---|---|---|---|
    "#
  );

  let mut rows = Vec::with_capacity(quotes.len());

  for quote in quotes.iter().take(2) {
    let token = quote.input_mint;
    let token_amount = quote.in_amount;
    let sol_amount = lamports_to_sol(quote.out_amount);
    let raydium_link = format!("https://raydium.io/swap/?inputMint={:?}&outputMint=sol", token);

    let row = format!(
      r#"
      |{}|{}|{}|{}|
      "#,
      token, token_amount, sol_amount, raydium_link
    );

    rows.push(row);
  }

  let body = rows.into_iter().reduce(|body, row| body + &row + &"\n").unwrap();
  let message = header + &body;
  let mut options = Options::default();
  
  options.extension.table = true;
  markdown_to_html(&message, &options)
}
