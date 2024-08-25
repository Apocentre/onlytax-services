use eyre::Result;
use diesel::{sql_query, sql_types::{VarChar, Int4}};
use diesel_async::RunQueryDsl;
use crate::{connection::PostgresConnection, models::collect_transaction::NewCollectTransaction};

impl PostgresConnection {
  pub async fn upsert_collect_transaction(&mut self, data: NewCollectTransaction<'_>) -> Result<()> {
    let query = sql_query(
      "
      INSERT INTO tokens (
        withdraw_withheld_authority, token, batch_size, tx_signature
      )
      VALUES($1, $2, $3, $4)
      ON CONFLICT (address) DO NOTHING
      "
    )
    .bind::<VarChar, _>(data.withdraw_withheld_authority)
    .bind::<VarChar, _>(data.token)
    .bind::<Int4, _>(data.batch_size)
    .bind::<VarChar, _>(data.tx_signature);

    query.execute(self.borrow_mut()).await?;

    Ok(())
  }
}
