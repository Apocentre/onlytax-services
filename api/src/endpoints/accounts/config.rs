use std::rc::Rc;
use actix_web::web;
use crate::middlewares::ec_auth::EcAuthnMiddlewareFactory;
use super::{connect, get_account_data};

pub fn config(ec_auth_middleware: Rc<EcAuthnMiddlewareFactory>) -> impl FnOnce(&mut web::ServiceConfig) {
  move |cfg: &mut web::ServiceConfig| {
    cfg.service(
      web::resource("")
      .route(web::post().to(connect::exec)).wrap(Rc::clone(&ec_auth_middleware))
    );

    cfg.service(
      web::resource("/{account}")
      .route(web::get().to(get_account_data::exec))
    );
  }
}
