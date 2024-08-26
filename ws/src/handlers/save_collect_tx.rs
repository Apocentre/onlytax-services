use std::sync::Arc;
use eyre::Result;
use onlytax_storage::models::collect_transaction::NewCollectTransaction;
use serde::Deserialize;
use crate::utils::store::Store;


#[derive(Deserialize)]
pub struct Request {
  pub access_token: String,
  pub withdraw_withheld_authority: String,
  pub token: String,
  pub batch_count: i32,
  pub tx_signature: String,
}

pub async fn exec(
  store: Arc<Store>,
  request: Request,
) -> Result<()> {
  let mut postgres = store.pg_pool.connection().await?;
  postgres.upsert_collect_transaction(NewCollectTransaction {
    withdraw_withheld_authority: &request.withdraw_withheld_authority,
    token: &request.token,
    batch_count: request.batch_count,
    tx_signature: &request.tx_signature,
  }).await?;

  Ok(())
}
