use dotenv::dotenv;
use infrastructure::db;
use std::error::Error;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate serde_derive;
extern crate serde_json;

mod api_error;
mod application;
mod domain;
mod infrastructure;

#[path = "application/grpc/pixgrpc.rs"]
mod api_grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  env_logger::init();
  db::init();
  Ok(())
}
