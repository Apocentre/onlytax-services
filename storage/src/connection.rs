use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Object};

pub struct PostgresConnection(Object<AsyncPgConnection>);

impl PostgresConnection {
  pub fn new(conn: Object<AsyncPgConnection>) -> Self {
    Self(conn)
  }

  pub fn borrow(&mut self) -> &AsyncPgConnection {
    &self.0
  }

  pub fn borrow_mut(&mut self) -> &mut AsyncPgConnection {
    &mut self.0
  }
}
