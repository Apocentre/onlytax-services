use std::time::Duration;
use deadpool::Runtime;
use diesel_async::{
  AsyncPgConnection,
  pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool, ManagerConfig},
};
use diesel::{ConnectionError, ConnectionResult};
use futures_util::{future::BoxFuture, FutureExt};
use eyre::Result;
use super::connection::PostgresConnection;

pub struct ConnectionPool(Pool<AsyncPgConnection>);

impl ConnectionPool {
  pub async fn new(db_uri: &str) -> Self {
    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(establish_connection);

    // First we have to construct a connection manager with our custom `establish_connection` function
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(db_uri, config);

    // From that connection we can then create a pool, here given with some example settings.
    //
    // This creates a TLS configuration that's equivalent to `libpq'` `sslmode=verify-full`, which
    // means this will check whether the provided certificate is valid for the given database host.
    //
    // `libpq` does not perform these checks by default (https://www.postgresql.org/docs/current/libpq-connect.html)
    // If you hit a TLS error while conneting to the database double check your certificates
    let pool = Pool::builder(manager)
    .max_size(10)
    .wait_timeout(Some(Duration::from_secs(60)))
    .create_timeout(Some(Duration::from_secs(60)))
    .recycle_timeout(Some(Duration::from_secs(60)))
    .runtime(Runtime::Tokio1)
    .build()
    .unwrap();
  
    Self(pool)
  }

  pub async fn connection(&self) -> Result<PostgresConnection> {
    let conn = self.0.get().await?;
    Ok(PostgresConnection::new(conn))
  }
}

fn establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
  let fut = async {
    let rustls_config = rustls::ClientConfig::builder()
    .with_root_certificates(rustls::RootCertStore::empty())
    .with_no_client_auth();
    let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);

    let (client, conn) = tokio_postgres::connect(config, tls).await
    .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;

    tokio::spawn(async move {
      if let Err(e) = conn.await {
        println!("Database connection failed: {e}");
      }
    });

    AsyncPgConnection::try_from(client).await
  };

  fut.boxed()
}
