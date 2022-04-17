use std::{env, error::Error};

use crate::{
  domain::model::{account::AccountModel, bank::BankModel, pix_key::PixKeyRepositoryInterface},
  infrastructure::repository::pix::PixkeyRepositoryDb,
};

mod application;
mod domain;
mod infrastructure;

#[path = "application/grpc/pixgrpc.rs"]
mod api_grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv::from_filename(".env.local").ok();
  env_logger::init();

  //create bank test
  //let bank = BankModel::new("BAC".to_string(), "ITAU".to_string());
  //<PixkeyRepositoryDb as PixKeyRepositoryInterface>::add_bank(bank).await;
  //create account test
  ///let account = AccountModel::new(bank, "123456".to_string(), "fernando".to_string());
  //<PixkeyRepositoryDb as PixKeyRepositoryInterface>::add_account(account).await;
  //
  // log::info!("Starting grpc Server");
  // let pix_service = api_grpc::MyPix {};
  // api_grpc::server_grpc(pix_service).await;

  // print!("{:?}", env);
  Ok(())
}
