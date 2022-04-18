pub mod schema;

use std::env;

use diesel::r2d2::ManageConnection;
use diesel::Connection;
use diesel::{
  pg::PgConnection,
  r2d2::{ConnectionManager, Pool},
};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool_pg() -> PgPool {
  let db_url = env::var("DATABASE_URL_DEV").expect("Can't get DB URL");
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  Pool::builder()
    .build(manager)
    .expect("Failed to create pool")
}

pub fn create_connection_pool<T>(env: &str) -> Pool<ConnectionManager<T>>
where
  T: ManageConnection + Connection,
{
  match env == "test" {
    true => {
      let db_url = env::var("DATABASE_URL_TEST").expect("Can't get DB URL");
      let manager = ConnectionManager::<T>::new(db_url);
      let db = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
      db
    }
    false => {
      let db_url = env::var("DATABASE_URL_DEV").expect("Can't get DB URL");
      let manager = ConnectionManager::<T>::new(db_url);
      let db = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
      db
    }
  }
}
