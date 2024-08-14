use std::sync::Arc;
use log::error;
use socketioxide::{extract::{Data, SocketRef, State}, SocketIo};
use crate::utils::store::Store;

use super::collect_fees;

pub fn register_handlers(io: Arc<SocketIo>) { 
  io.ns("/", |s: SocketRef, store: State<Arc<Store>>| {
    let store = Arc::clone(&store);

    s.on("collect", |s: SocketRef, Data::<(String, String)>((mint, withdraw_withheld_authority))| async move {
      if let Err(err) = s.join([withdraw_withheld_authority.clone()]) {
        error!("Could not join the {} room {}", withdraw_withheld_authority, err);
      };

      if let Err(err) = collect_fees::exec(store, s, &mint, &withdraw_withheld_authority).await {
        error!("Error while collecting fees for withheld authority {}  {}", withdraw_withheld_authority, err);
      }
    });
  });
}
