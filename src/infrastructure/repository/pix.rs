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
        PixKeyP::status().set("active".to_string()),
        vec![],
      )
      .exec()
      .await;
    pix_key
  }

  async fn find_key_by_kind(kind: String, key: String) -> Result<PixKeyPData, String> {
    let client = PrismaClient::new().await;
    //find pix_key
    let pix_key = client
      .pix_key_p()
      .find_first(vec![
        PixKeyP::key().equals(key.clone()),
        PixKeyP::kind().equals(kind.clone()),
      ])
      .exec()
      .await;
    //verify
    let pix_key_res = pix_key.key.clone();
    let pix_key = match pix_key_res == key {
      true => Ok(pix_key),
      false => Err(String::from("invalid pix_key")),
    };
    pix_key
  }

  async fn add_bank(bank: &BankP) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  async fn add_account(account: &AccountP) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  async fn find_account(id: String) -> Result<AccountPData, String> {
    let client = PrismaClient::new().await;
    //find account
    let account = client
      .account_p()
      .find_unique(AccountP::id().equals(id.clone()))
      .exec()
      .await;
    //verify
    let account_id = account.id.clone();
    let account = if let true = account_id == id {
      Ok(account)
    } else {
      Err(String::from("Not found account"))
    };
    account
  }
}
