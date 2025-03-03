use std::{sync::Arc, time::Duration};
use log::error;
use teloxide::{prelude::*, types::{LinkPreviewOptions, ParseMode}, utils::command::BotCommands};
use tokio::time;
use crate::utils::store::Store;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
  #[command(description = "display this text.")]
  Help,
  #[command(description = "Enable treasury account notifications")]
  Enable,
  #[command(description = "Disable treasury account notifications")]
  Disable,
}

pub struct TreasuryBot {
  store: Arc<Store>,
}

impl TreasuryBot {
  pub fn new(store: Arc<Store>) -> Self {
    Self {store}
  }

  pub async fn start(&self) {
    let bot = Bot::new(self.store.config.teloxide_token.clone());
    let store = Arc::clone(&self.store);

    let handler = move |bot: Bot, msg: Message, cmd: Command| {
      let store = Arc::clone(&store);
      Self::answer(store, bot, msg, cmd)
    };

    self.poll_token_account();

    Command::repl(bot, handler).await;
  }

  fn poll_token_account(&self) {
    let store = Arc::clone(&self.store);
    let poll_interval_secs = store.config.poll_interval_secs;
    let treasury = store.config.treasury.clone();

    tokio::spawn(async move {
      let mut interval = time::interval(Duration::from_secs(poll_interval_secs));

      loop {
        if !store.storage.enabled() {
          continue;
        }

        interval.tick().await;

        let Ok(token_accounts) = store.helius_api.fetch_token_accounts_by_owner(&treasury).await else {
          error!("Failed to fetch token accounts");
          continue;
        };

        println!("Token Accounts {:?}", token_accounts);
      }
    });
  }
  
  async fn answer(store: Arc<Store>, bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
      Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
      Command::Enable => {
        store.storage.enable();
        bot.send_message(msg.chat.id, "Enabled!").await?
      },
      Command::Disable => {
        store.storage.disable();
        bot.send_message(msg.chat.id, "Disabled!").await?
      }
    };

    Ok(())
  }

}
