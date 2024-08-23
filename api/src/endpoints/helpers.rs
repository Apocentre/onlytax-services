use eyre::Result;
use actix_web::HttpResponse;
use serde::Serialize;
use onlytax_storage::models::count::Count;
use crate::utils::error::Error;

#[derive(Serialize)]
pub struct BaseResponse<T: Serialize> {
  pub count: usize,
  pub skip: i64,
  pub limit: i64,
  pub result: T,
}

pub fn create_read_response_count<T: Serialize>(result: Result<(Count, Vec<T>)>, skip: i64, limit: i64) -> Result<HttpResponse, Error> {
  result
  .map(|result| {
    HttpResponse::Ok()
      .json(BaseResponse {
        count: result.0.count as usize,
        result: result.1,
        skip: skip,
        limit: limit,
      })
  })
  .map_err(|err| err.into())
}
