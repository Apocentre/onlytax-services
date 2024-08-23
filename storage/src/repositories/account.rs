use eyre::Result;
use diesel::{sql_query, sql_types::VarChar};
use diesel_async::RunQueryDsl;
use crate::{connection::PostgresConnection, models::account::Account};

impl PostgresConnection {
  pub async fn upsert_account(&mut self, address: &str) -> Result<()> {
    let query = sql_query(
      "
      INSERT INTO accounts (
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

  pub async fn read_account_data(&mut self, address: &str) -> Result<Account> {
    let query = sql_query(format!(
      "
        SELECT *
        FROM accounts
        WHERE address = '{}'
      ",
      address,
    ));

    let record = query.get_result::<Account>(self.borrow_mut()).await?;

    Ok(record)
  }

}
