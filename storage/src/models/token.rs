use diesel::{
  prelude::*, sql_types::VarChar,
};
use serde::Serialize;

#[derive(Queryable, QueryableByName, Serialize, Debug)]
pub struct Token {
  #[diesel(sql_type = VarChar)]
  pub address: String,
}
