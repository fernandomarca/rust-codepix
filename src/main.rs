use application::usecase::pix::PixUseCase;
use dotenv::dotenv;
use infrastructure::db::create_connection_pool_pg;
use std::{env, error::Error};

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate serde_derive;
extern crate serde_json;

mod application;
mod domain;
mod infrastructure;

#[path = "application/grpc/pixgrpc.rs"]
mod api_grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();

  let _env = env::var("env").expect("Can't get env");

  let conn = create_connection_pool_pg().get()?;

  // let result = PixUseCase::register_key(
  //   &conn,
  //   "email".to_string(),
  //   "fernandomarca@hotmail.com".to_string(),
  //   "2".to_string(),
  // );
  // print!("result: {:?}", result);
  // let account = PixUseCase::find_account(&conn, "1".to_string());
  // print!("result: {:?}", account);
  //
  dotenv::from_filename(".env").ok();

  Ok(())
}
