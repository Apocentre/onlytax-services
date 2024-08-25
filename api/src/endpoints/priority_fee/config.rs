use std::rc::Rc;
use actix_web::web;
use crate::middlewares::jwt_auth::JwtAuthnMiddlewareFactory;
use super::get_priority_fee

pub fn config(jwt_authn_middleware: Rc<JwtAuthnMiddlewareFactory>) -> impl FnOnce(&mut web::ServiceConfig) {
  move |cfg: &mut web::ServiceConfig| {
    cfg.service(
      web::resource("")
      .route(web::post().to(get_priority_fee::exec)).wrap(Rc::clone(&jwt_authn_middleware))
    );
  }
}
