use crate::{
  domain,
  infrastructure::{
    prisma_db::db::{AccountPData, PixKeyPData},
    repository::pix::PixkeyRepositoryDb,
  },
};
use domain::model::pix_key::PixKeyRepositoryInterface;
use std::error::Error;

pub struct PixUseCase {}

impl PixUseCase {
  #[tokio::main]
  pub async fn register_key(key: String, kind: String, account_id: String) -> PixKeyPData {
    let account =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(account_id.clone()).await;
    //
    //let id = account.id;
    let pix_key =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::register_key(key, kind, account_id).await;
    pix_key
  }
  #[tokio::main]
  pub async fn find_account(id: String) -> AccountPData {
    let account = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(id).await;
    account
  }
}
