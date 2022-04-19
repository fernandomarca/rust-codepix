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

// use crate::api_error::ApiError;
// use diesel::pg::PgConnection;
// use diesel::r2d2::ConnectionManager;
// use lazy_static::lazy_static;
// use r2d2;
// use std::env;

// type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// embed_migrations!();

// lazy_static! {
//     static ref POOL: Pool = {
//         let db_url = env::var("DATABASE_URL").expect("Database url not set");
//         let manager = ConnectionManager::<PgConnection>::new(db_url);
//         Pool::new(manager).expect("Failed to create db pool")
//     };
// }

// pub fn init() {
//     info!("Initializing DB");
//     lazy_static::initialize(&POOL);
//     let conn = connection().expect("Failed to get db connection");
//     embedded_migrations::run(&conn).unwrap();
// }

// pub fn connection() -> Result<DbConnection, ApiError> {
//     POOL.get()
//         .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
// }
