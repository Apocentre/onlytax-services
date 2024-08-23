use std::{
  future::{ready, Ready as StdReady},
  rc::Rc, sync::Arc,
};
use actix_web::{
  HttpMessage,
  dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
  FromRequest,
  Error,
  HttpRequest,
  error::ErrorUnauthorized,
};
use log::error;
use futures_util::future::{LocalBoxFuture, ok, err, Ready};
use crate::services::auth::Auth;

#[derive(Debug, Clone)]
pub struct JwtAuthData {
  pub account: String,
}

impl FromRequest for JwtAuthData {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    req.extensions()
    .get::<JwtAuthData>()
    .map(|auth_data| auth_data.clone())
    .map(ok)
    .unwrap_or_else(|| err(ErrorUnauthorized("not authorized")))
  }
}

pub struct JwtAuthnMiddlewareFactory {
  auth: Arc<Auth>,
}

impl JwtAuthnMiddlewareFactory {
  pub fn new(auth: Arc<Auth>) -> Self {
    let auth = Arc::clone(&auth);

    Self {auth}
  }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuthnMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthnMiddleware<S>;
    type Future = StdReady<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(JwtAuthnMiddleware {
        service: Rc::new(service),
        auth: self.auth.clone(),
      }))
    }
}

pub struct JwtAuthnMiddleware<S> {
  service: Rc<S>,
  auth: Arc<Auth>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthnMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let srv = self.service.clone();
    let auth = self.auth.clone();
    
    Box::pin(
      async move {
        let headers = req.headers();
        let bearer = headers.get("Authorization").ok_or(ErrorUnauthorized("Unauthorized"))?;
        
        let mut iter = bearer
        .to_str()
        .map_err(|_| ErrorUnauthorized("Unauthorized"))?
        .split_whitespace();
        
        if let Some(prefix) = iter.next() {
          if prefix != "Bearer" {
            return Err(ErrorUnauthorized("Unauthorized"))
          }
        }

        let access_token = if let Some(access_token) = iter.next() {
          access_token
        } else {
          return Err(ErrorUnauthorized("Unauthorized"))
        };
        
        match auth.verify_jwt(&access_token) {
          Ok(account) => {
            // make the user available to the downstream handlers
            req.extensions_mut().insert(JwtAuthData {account});
  
            return Ok(srv.call(req).await?)
          },
          Err(error) => {
            error!("{}", error);
            return Err(ErrorUnauthorized("Unauthorized"))
          }
        }
      }
    )
  }
}
