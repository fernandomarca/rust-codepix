use crate::application::usecase::transaction::TransactionUseCase;
use api_error::ApiError;
use domain::model::{account::NewAccount, bank::NewBank, pix_key::PixKeyRepositoryInterface};
use dotenv::dotenv;
use infrastructure::{db, repository::pix::PixkeyRepositoryDb};

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
async fn main() -> Result<(), ApiError> {
  dotenv().ok();
  env_logger::init();
  db::init();
  //let bank = NewBank::new(String::from("ITAU"), String::from("ITAU"));
  //<PixkeyRepositoryDb as PixKeyRepositoryInterface>::add_bank(bank)?;
  //
  // let account = NewAccount::new(
  //   String::from("Rhayza"),
  //   String::from("654321"),
  //   String::from("75dd9b70-2985-4b80-86d7-59360d0f3816"),
  // );
  // //
  // <PixkeyRepositoryDb as PixKeyRepositoryInterface>::add_account(account)?;
  //<PixkeyRepositoryDb as PixKeyRepositoryInterface>::register_key(
  //String::from("fernandomarca@hotmail"),
  //String::from("email"),
  //String::from("27baa390-dcef-456c-b92e-78611283930f"),
  //)?;

  // TransactionUseCase::register(
  //   String::from("27baa390-dcef-456c-b92e-78611283930f"),
  //   100.56,
  //   String::from("e34e7def-35cf-4bd8-8822-3d5614854b47"),
  //   String::from("email"),
  //   String::from("teste tr"),
  //   None,
  // )?;
  Ok(())
}
