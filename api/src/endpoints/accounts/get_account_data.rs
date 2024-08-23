use actix_web::{web, HttpResponse};
use serde::Deserialize;
use eyre::Result;
use crate::utils::{error::Error, store::Store};

#[derive(Deserialize)]
pub struct Params {
  pub account: String,
}

pub async fn exec(
  store: web::Data<Store>,
  params: web::Path<Params>,
) -> Result<HttpResponse, Error> {
  let account = &params.account;
  let mut postgres = store.pg_pool.connection().await?;
  let account_data = postgres.read_account_data(account).await?;

  Ok(HttpResponse::Ok().json(account_data))
}
