use std::{str::FromStr, sync::Arc};
use eyre::Result;
use futures::StreamExt;
use log::error;
use serde::Serialize;
use socketioxide::extract::SocketRef;
use solana_sdk::pubkey::Pubkey;
use crate::utils::store::Store;

#[derive(Serialize)]
struct SerializedTx {
  data: Vec<u8>,
}

pub async fn exec(
  store: Arc<Store>,
  socket: SocketRef,
  mint: &str,
  withdraw_withheld_authority: &str,
) -> Result<()> {
  let room = withdraw_withheld_authority.to_string();
  let mint = Pubkey::from_str(mint)?;
  let withdraw_withheld_authority_key = Pubkey::from_str(withdraw_withheld_authority)?;
  let mut stream = store.fee_collector.collect(&mint, &withdraw_withheld_authority_key);

  while let Some(Ok(data)) = stream.next().await {
    if let Err(err) = socket.emit(room.clone(), SerializedTx {data}) {
      error!("failed to push new_batch for authority {} {}", withdraw_withheld_authority, err);
    }
  }

  Ok(())
}
