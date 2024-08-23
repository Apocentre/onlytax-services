use actix_web::ResponseError;
use thiserror::Error;

// A generc error wrapper makes Actix expected errors comatbible with the eyre::Report error so we can directly use the ? operator
#[derive(Error, Debug)]
pub enum Error {
  #[error("Generic Error: {0}")]
  GenericError(String),
}

impl ResponseError for Error {}

impl From<eyre::ErrReport> for Error {
  fn from(error: eyre::ErrReport) -> Self {
    Error::GenericError(format!("{:?}", error))
  }
}

// Executed the provided function and converts the Result into eyre::Result
#[macro_export]
macro_rules! map_err {
  ($fun:expr) => {
    $fun.map_err(|e| eyre!(Box::new(e)))
  }
}
