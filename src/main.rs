use diesel::{
  r2d2::{ConnectionManager, PooledConnection},
  PgConnection,
};
use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use infrastructure::db::{create_connection_pool_pg, PgPool};
use std::{env, error::Error};

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
