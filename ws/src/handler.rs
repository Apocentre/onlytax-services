use std::sync::Arc;
use socketioxide::{extract::{Data, SocketRef}, SocketIo};

pub fn register_handlers(io: Arc<SocketIo>) {
  io.ns("/", |s: SocketRef| {
    s.on("collect", |s: SocketRef, Data::<String>(withdraw_withheld_authority)| {
      s.join([withdraw_withheld_authority]).unwrap();
    });
  });
}
