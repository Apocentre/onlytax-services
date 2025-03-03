use std::{sync::Arc, env, panic, process};
use dotenv;
use env_logger::Env;
use eyre::Result;
use tg_treasuty_bot::{bot::TreasuryBot, utils::store::Store};

#[tokio::main]
async fn main() -> Result<()> {
  let orig_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info| {
    orig_hook(panic_info);
    process::exit(1);
  }));

  if env::var("ENV").unwrap() == "development" {
    dotenv::from_filename(".env").expect("cannot load env from a file");
  }

  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let store = Arc::new(Store::new().await);
  let treasury_bot = TreasuryBot::new(store);
  treasury_bot.start().await;

  Ok(())
}
