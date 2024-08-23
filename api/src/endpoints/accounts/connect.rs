use actix_web::{web, HttpResponse};
use eyre::Result;
use serde::Serialize;
use crate::{
  middlewares::ec_auth::EcAuthData,
  utils::{error::Error, store::Store},
};

#[derive(Serialize)]
pub struct Response {
  jwt: String,
}

pub async fn exec(store: web::Data<Store>, auth: EcAuthData) -> Result<HttpResponse, Error> {
  let mut postgres = store.pg_pool.connection().await?;
  
  // creates account if it doesn't already exist
  postgres.upsert_account(&auth.account).await?;

  let jwt = store.auth.create_jwt(&auth.account)?;

  Ok(HttpResponse::Ok().json({
    Response {jwt}
  }))
}
