use chrono::{Days, Utc};
use eyre::{ContextCompat, Result};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: String,
  iat: i64,
  exp: i64,
}

pub struct Auth {
  encoding_key: EncodingKey,
  decoding_key: DecodingKey,
}

impl Auth {
  pub fn new(key: &str) -> Self {
    let encoding_key = EncodingKey::from_secret(&hex::decode(key).unwrap());
    let decoding_key = DecodingKey::from_secret(&hex::decode(key).unwrap());
    
    Self {
      encoding_key,
      decoding_key,
    }
  }

  pub fn create_jwt(&self, account: &str) -> Result<String> {
    let now_utc = Utc::now();

    // create claims valid for 1 month
    let claims = Claims {
      sub: account.to_string(),
      iat: now_utc.timestamp(),
      exp: now_utc.checked_add_days(Days::new(30)).context("should add days")?.timestamp(),
    };

    let jwt = encode(&Header::default(), &claims, &self.encoding_key)?;

    Ok(jwt)
  }

  pub fn verify_jwt(&self, jwt: &str) -> Result<String> {
    // decode will check if its signature is invalid. It will also check if token has expired
    let token_data = decode::<Claims>(&jwt, &self.decoding_key, &Validation::default())?;

    Ok(token_data.claims.sub)
  }
}
