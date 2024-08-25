use eyre::Result;
use diesel::{sql_query, sql_types::VarChar};
use diesel_async::RunQueryDsl;
use crate::connection::PostgresConnection;

impl PostgresConnection {
  pub async fn upsert_token(&mut self, address: &str) -> Result<()> {
    let query = sql_query(
      "
      INSERT INTO tokens (
        address
      )
      VALUES($1)
      ON CONFLICT (address) DO NOTHING
      "
    )
    .bind::<VarChar, _>(address);

    query.execute(self.borrow_mut()).await?;

    Ok(())
  }
}
