use std::{str::FromStr, sync::Arc};
use eyre::Result;
use futures::StreamExt;
use log::error;
use serde::Serialize;
use socketioxide::extract::SocketRef;
use solana_sdk::pubkey::Pubkey;
use crate::utils::store::Store;

#[derive(Serialize)]
struct Response {
  tx: Option<Vec<u8>>,
  batch_size: usize,
  batch_count: usize,
}

pub async fn exec(
  store: Arc<Store>,
  socket: SocketRef,
  mint: &str,
  withdraw_withheld_authority: &str,
) -> Result<()> {
  let mut postgres = store.pg_pool.connection().await?;
  postgres.upsert_token(mint).await?;

  let room = format!("{}-{}", mint, withdraw_withheld_authority);
  let mint = Pubkey::from_str(mint)?;
  let withdraw_withheld_authority_key = Pubkey::from_str(withdraw_withheld_authority)?;
  let mut stream = store.fee_collector.collect(&mint, &withdraw_withheld_authority_key);

  while let Some(Ok(item)) = stream.next().await {
    let response = Response {
      tx: Some(item.0),
      batch_size: item.1,
      batch_count: item.2,
    };
    if let Err(err) = socket.emit(room.clone(), response) {
      error!("failed to push new_batch for authority {} {}", withdraw_withheld_authority, err);
    }
  }

  // This msg will indicate the end of the stream. Usefull for WS clients to know when
  // the transactions are all consumed
  let response = Response {
    tx: None,
    batch_size: 0,
    batch_count: 0,
  };
  if let Err(err) = socket.emit(room.clone(), response) {
    error!("failed to push batch-complete for authority {} {}", withdraw_withheld_authority, err);
  }

  Ok(())
}
