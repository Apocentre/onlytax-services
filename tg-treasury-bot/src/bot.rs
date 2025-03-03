use std::sync::Arc;
use log::error;
use teloxide::{prelude::*, types::{LinkPreviewOptions, ParseMode}, utils::command::BotCommands};
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
      answer(store, bot, msg, cmd)
    };

    Command::repl(bot, handler).await;
  }
}

async fn answer(store: Arc<Store>, bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
  match cmd {
    Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
    Command::Enable => {
      let chat_id = msg.chat.id.to_string();
      // TODO: poll treasury token account summary
      bot.send_message(msg.chat.id, "Enabled!").await?
    },
    Command::Disable => {
      let chat_id = msg.chat.id.to_string();
      // // TODO: stop polling treasury token account summary
      bot.send_message(msg.chat.id, "Disabled!").await?
    }
  };

  Ok(())
}
