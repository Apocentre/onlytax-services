use actix_web::{web, HttpResponse};
use eyre::Result;
use crate::{
  middlewares::jwt_auth::JwtAuthData,
  utils::{error::Error, store::Store}
};

pub async fn exec(
  store: web::Data<Store>,
  _: JwtAuthData,
) -> Result<HttpResponse, Error> {
  let response = store.helius_api.fetch_priority_fee().await?;

  Ok(HttpResponse::Ok().json(response.priority_fee_levels))
}
