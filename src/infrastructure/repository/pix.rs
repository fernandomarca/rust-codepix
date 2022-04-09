use crate::domain::model::pix_key::PixKeyRepositoryInterface;
use crate::infrastructure::prisma_db::db::{
  AccountP, AccountPData, BankP, PixKeyP, PixKeyPData, PrismaClient,
};
use async_trait::async_trait;
use chrono::Utc;
use std::error::Error;
pub struct PixkeyRepositoryDb {}

#[async_trait]
impl PixKeyRepositoryInterface for PixkeyRepositoryDb {
  async fn register_key(key: String, kind: String, account_id: String) -> PixKeyPData {
    let client = PrismaClient::new().await;
    let now = Utc::now().to_string();
    //
    let pix_key = client
      .pix_key_p()
      .create_one(
        PixKeyP::kind().set(kind),
        PixKeyP::key().set(key),
        PixKeyP::created_at().set(now),
        PixKeyP::account().link(AccountP::id().equals(account_id)),
        vec![],
      )
      .exec()
      .await;
    pix_key
  }

  async fn find_key_by_kind(key: String, kind: String) -> Result<PixKeyP, Box<dyn Error>> {
    todo!()
  }

  async fn add_bank(bank: &BankP) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  async fn add_account(account: &AccountP) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  async fn find_account(id: String) -> AccountPData {
    let client = PrismaClient::new().await;
    let account = client
      .account_p()
      .find_unique(AccountP::id().equals(id))
      .exec()
      .await;
    account
  }
}
