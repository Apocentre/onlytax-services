use serde::Serialize;
use diesel::{
  prelude::*, sql_types::BigInt,
};

#[derive(QueryableByName, Serialize, Debug)]
pub struct Count {
  #[diesel(sql_type = BigInt)]
  pub count: i64,
}
