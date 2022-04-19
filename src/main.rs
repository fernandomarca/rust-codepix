use application::usecase::pix::PixUseCase;
use diesel::{
  r2d2::{ConnectionManager, PooledConnection},
  PgConnection,
};
use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use infrastructure::db::{create_connection_pool_pg, PgPool};
use std::{env, error::Error};

use crate::domain::model::{
  account::NewAccount, bank::NewBank, pix_key::PixKeyRepositoryInterface,
};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate serde_derive;
extern crate serde_json;

embed_migrations!();

mod application;
mod domain;
mod infrastructure;

#[path = "application/grpc/pixgrpc.rs"]
mod api_grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();

  let _env = env::var("env").expect("Can't get env");

  let pool = create_connection_pool_pg();
  run_migrations(&pool);
  // let result = PixUseCase::register_key(
  //   &conn,
  //   "email".to_string(),
  //   "fernandomarca@hotmail.com".to_string(),
  //   "2".to_string(),
  // );
  // print!("result: {:?}", result);
  // let account = PixUseCase::find_account(&get_connection(&pool), "1".to_string());
  // print!("result: {:?}", account);
  use infrastructure::repository::pix::PixkeyRepositoryDb;
  // let bank = NewBank::new("ITA".to_string(), "itau".to_string());
  // <PixkeyRepositoryDb as PixKeyRepositoryInterface>::add_bank(&get_connection(&pool), bank);
  let account = NewAccount::new(
    "fernando".to_string(),
    "12345".to_string(),
    "b3a7a964-787a-4928-9e9d-2bda063c3a0d".to_string(),
  );
  <PixkeyRepositoryDb as PixKeyRepositoryInterface>::add_account(&get_connection(&pool), account);
  dotenv::from_filename(".env").ok();

  Ok(())
}

fn run_migrations(pool: &PgPool) {
  let conn = pool.get().expect("Can't get DB connection");
  embedded_migrations::run(&conn).expect("Failed to run database migrations");
}

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

fn get_connection(pool: &PgPool) -> Conn {
  pool.get().expect("Can't get DB connection")
}
