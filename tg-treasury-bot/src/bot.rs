use std::{sync::Arc, time::Duration};
use log::error;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use teloxide::{prelude::*, types::{LinkPreviewOptions, ParseMode}, utils::command::BotCommands};
use tokio::time;
use crate::{jupiter::Jupiter, message::format_message, utils::store::Store};

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
  bot: Bot,
}

impl TreasuryBot {
  pub fn new(store: Arc<Store>) -> Self {
    let bot = Bot::new(store.config.teloxide_token.clone());

    Self {
      store,
      bot,
    }
  }

  pub async fn start(&self) {
    let store = Arc::clone(&self.store);

    let handler = move |bot: Bot, msg: Message, cmd: Command| {
      let store = Arc::clone(&store);
      Self::answer(store, bot, msg, cmd)
    };

    self.poll_token_account();

    Command::repl(self.bot.clone(), handler).await;
  }

  fn poll_token_account(&self) {
    let store = Arc::clone(&self.store);
    let poll_interval_secs = store.config.poll_interval_secs;
    let slippage_bps = store.config.slippage_bps;
    let treasury = store.config.treasury.clone();
    let bot = self.bot.clone();

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

        let mut quotes = Vec::with_capacity(token_accounts.len());

        for ta in &token_accounts {
          let Ok(quote) = Jupiter::quote(&ta.mint, ta.amount, slippage_bps).await.inspect_err(|err| {
            error!("Error fetching quote for {} {:?}", ta.address, err);
          }) else {
            continue;
          };

          quotes.push(quote);
        }

        // keep quotes that are larger than 1 SOL
        let mut quotes = quotes.into_iter().filter(|q| q.out_amount >= LAMPORTS_PER_SOL).collect::<Vec<_>>();
        quotes.sort_by(|a, b| b.out_amount.cmp(&a.out_amount));

        let messages = if !token_accounts.is_empty() {
          format_message(quotes)
        } else {
          continue;
        };

        let link_preview_options = serde_json::from_str::<LinkPreviewOptions>(r#"{"is_disabled": true}"#).unwrap();
        let chat_id = ChatId(store.storage.chat_id());

        for message in messages {
          if let Err(err) = bot.send_message(chat_id, &message)
            .parse_mode(ParseMode::MarkdownV2)
            .link_preview_options(link_preview_options.clone()).await 
          {
            error!("Could not send new trade to chat {}: {}", chat_id, err);
          }
        }
      }
    });
  }
  
  async fn answer(store: Arc<Store>, bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
      Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
      Command::Enable => {
        store.storage.enable();
        store.storage.set_chat_id(msg.chat.id.0);
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
