use diesel::{
  prelude::*, sql_types::{VarChar, Int4, Timestamp},
};
use serde::{Deserialize, Serialize};
use chrono::{
  naive::serde::ts_milliseconds::serialize as to_milli_ts, NaiveDateTime,
};
use crate::schema::collect_transactions;


#[derive(Queryable, QueryableByName, Serialize, Debug)]
#[diesel(table_name = collect_transactions)]
pub struct CollectTransaction {
  pub id: i32,

  #[diesel(sql_type = VarChar)]
  pub withdraw_withheld_authority: String,

  #[diesel(sql_type = VarChar)]
  pub token: String,

  #[diesel(sql_type = Int4)]
  pub batch_count: i32,

  #[diesel(sql_type = VarChar)]
  pub tx_signature: String,

  #[diesel(sql_type = Timestamp)]
  #[serde(serialize_with = "to_milli_ts")]
  pub created_at: NaiveDateTime,
}


#[derive(Deserialize)]
pub struct NewCollectTransaction<'a> {
  pub withdraw_withheld_authority: &'a str,
  pub token: &'a str,
  pub batch_count: i32,
  pub tx_signature: &'a str,
}
