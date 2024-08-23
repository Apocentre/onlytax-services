use std::{
  future::{ready, Ready as StdReady}, rc::Rc,
};
use actix_web::{
  HttpMessage, dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
  FromRequest, Error, HttpRequest, error::ErrorUnauthorized,
};
use futures_util::future::{LocalBoxFuture, ok, err, Ready};
use chrono::{offset::Utc, Duration};
use crate::services::crypto::verify_sig;

#[derive(Debug, Clone)]
pub struct EcAuthData {
  pub account: String,
}

impl FromRequest for EcAuthData {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    req.extensions()
    .get::<EcAuthData>()
    .map(|auth_data| auth_data.clone())
    .map(ok)
    .unwrap_or_else(|| err(ErrorUnauthorized("not authorized")))
  }
}

pub struct EcAuthnMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for EcAuthnMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = EcAuthnMiddleware<S>;
    type Future = StdReady<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(EcAuthnMiddleware {
        service: Rc::new(service),
      }))
    }
}

pub struct EcAuthnMiddleware<S> {
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for EcAuthnMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let srv = self.service.clone();

    Box::pin(
      async move {
        let headers = req.headers();

        let auth_data = headers.get("X-Onlytax-Auth")
        .ok_or(ErrorUnauthorized("Unauthorized"))?
        .to_str()
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?
        .split(":")
        .collect::<Vec<_>>();

        let ts = auth_data.get(0).ok_or(ErrorUnauthorized("Unauthorized"))?
        .parse::<i64>()
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?;

        let account = auth_data.get(1).ok_or(ErrorUnauthorized("Unauthorized"))?;

        // A sig is valid for one day
        let now = Utc::now().timestamp_millis();
        if now - ts > Duration::days(1).num_milliseconds() {
          return Err(ErrorUnauthorized("Unauthorized"))
        }

        let sig = auth_data.get(2).ok_or(ErrorUnauthorized("Unauthorized"))?;
        let msg = format!("Onlytax Auth:{}", ts);
        let msg = msg.as_bytes();

        verify_sig(&msg, account.as_bytes(), sig).map_err(|_| ErrorUnauthorized("Unauthorized"))?;
        req.extensions_mut().insert(EcAuthData {
          account: account.to_string(),
        });

        Ok(srv.call(req).await?)
      }
    )
  }
}
