use api_grpc::{server_grpc, MyPix};
use cmd::cmd::{Action, CommandLineArgs};
use domain::model::transaction::TransactionRepositoryInterface;
use dotenv::dotenv;
use infrastructure::{db, repository::transaction::TransactionRepoDb};
use structopt::StructOpt;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate serde_derive;
extern crate serde_json;

mod api_error;
mod application;
mod cmd;
mod domain;
mod infrastructure;

#[path = "application/grpc/pixgrpc.rs"]
mod api_grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  env_logger::init();
  //<TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(String::from(
  //  "3f67b376-2c8c-41c8-b345-f7a25d7f440a",
  //));
  let CommandLineArgs { action, port } = CommandLineArgs::from_args();

  match action {
    Action::Start => {
      db::init();
      let pix_service = MyPix {};
      server_grpc(pix_service, port).await?;
    }
  };

  Ok(())
}
