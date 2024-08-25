use std::sync::Arc;
use log::error;
use socketioxide::{extract::{Data, SocketRef, State}, SocketIo};
use crate::utils::store::Store;
use super::{collect_fees, save_collect_tx};


pub fn register_handlers(io: Arc<SocketIo>) { 
  io.ns("/", |s: SocketRef, store: State<Arc<Store>>| {
    let store = Arc::clone(&store);
    let store_clone = Arc::clone(&store);

    s.on("collect", |s: SocketRef, Data::<(String, String)>((mint, withdraw_withheld_authority))| async move {
      let room = format!("{}-{}", mint, withdraw_withheld_authority);

      if let Err(err) = s.join([room.clone()]) {
        error!("Could not join the {} room {}", room, err);
      };

      if let Err(err) = collect_fees::exec(store, s, &mint, &withdraw_withheld_authority).await {
        error!("Error while collecting fees for room {}  {}", room, err);
      }
    });

    s.on("save-collect-tx", |_: SocketRef, Data::<save_collect_tx::Request>(request)| async move {
      let token = request.token.clone();
      let withdraw_withheld_authority = request.withdraw_withheld_authority.clone();

      if let Err(err) = save_collect_tx::exec(store_clone, request).await {
        error!(
          "Error while collecting fees for token and withheld authority {} {}: {}",
          token, withdraw_withheld_authority, err,
        );
      }
    });
  });
}
