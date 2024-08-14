use std::{env, panic, process, sync::Arc, time::Duration};
use axum::{routing::get, serve, Router};
use env_logger::Env;
use eyre::Result;
use socketioxide::{SocketIo, TransportType};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use onlytax_ws::{
  handlers::router::register_handlers, utils::store::Store
};

#[tokio::main]
async fn main() -> Result<()> {
  let orig_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info| {
    orig_hook(panic_info);
    process::exit(1);
  }));

  if env::var("ENV").unwrap() == "development" {
    dotenv::from_filename(".env").expect("cannot load env from a file");
  }

  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
  
  let store = Arc::new(Store::new().await);
  let port = store.config.port;
  let (layer, io) = SocketIo::builder()
  .transports([TransportType::Websocket])
  .ping_interval(Duration::from_secs(25))
  .ping_timeout(Duration::from_secs(20))
  .with_state(Arc::clone(&store)).build_layer();
  
  let io = Arc::new(io);

  register_handlers(Arc::clone(&io));

  async fn get_root() -> &'static str { "Hi!" }

  let app = Router::new()
  .route("/", get(get_root))
  .layer(
    ServiceBuilder::new()
    .layer(CorsLayer::permissive()) // Enable CORS policy
    .layer(layer),
  );

  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
  serve(listener, app).await.unwrap();

  Ok(())
}
