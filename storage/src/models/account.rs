use diesel::{
  prelude::*, sql_types::{VarChar, Timestamp},
};
use serde::Serialize;
use chrono::{
  naive::serde::ts_milliseconds::serialize as to_milli_ts, NaiveDateTime,
};


#[derive(Queryable, QueryableByName, Serialize, Debug)]
pub struct Account {
  #[diesel(sql_type = VarChar)]
  pub address: String,

  #[diesel(sql_type = Timestamp)]
  #[serde(serialize_with = "to_milli_ts")]
  pub created_at: NaiveDateTime,
}
