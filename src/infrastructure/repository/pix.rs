use crate::{
  domain::model::{
    account::AccountModel,
    bank::BankModel,
    pix_key::{PixKeyModel, PixKeyRepositoryInterface},
  },
  infrastructure::prisma_db::db::{AccountP, BankP, PixKeyP, PrismaClient},
};
use async_trait::async_trait;
use chrono::Utc;
use std::error::Error;
pub struct PixkeyRepositoryDb {}

#[async_trait]
impl PixKeyRepositoryInterface for PixkeyRepositoryDb {
  async fn register_key(
    key: String,
    kind: String,
    account_id: String,
  ) -> Result<PixKeyModel, String> {
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
    //
    let result = match &pix_key.id.trim().is_empty() {
      true => Err(String::from("error not register pixkey")),
      false => Ok(pix_key.into()),
    };
    result
  }

  async fn find_key_by_kind(kind: String, key: String) -> Result<PixKeyModel, String> {
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
      true => Ok(pix_key.into()),
      false => Err(String::from("invalid pix_key")),
    };
    pix_key
  }

  async fn add_bank(bank: BankModel) -> Result<(), Box<dyn Error>> {
    let client = PrismaClient::new().await;
    let name = bank.name;
    let code = bank.code;
    let created_at = bank.created_at.to_string();
    let result = client
      .bank_p()
      .create_one(
        BankP::name().set(name),
        BankP::code().set(code),
        BankP::created_at().set(created_at),
        vec![],
      )
      .exec()
      .await;
    if !result.id.trim().is_empty() {
      ()
    }
    Ok(())
  }

  async fn add_account(account: AccountModel) -> Result<(), Box<dyn Error>> {
    let client = PrismaClient::new().await;
    let owner_name = account.owner_name;
    let bank_id = account.bank_id.clone();
    let number = account.number;
    let created_at = account.created_at.to_string();
    let result = client
      .account_p()
      .create_one(
        AccountP::owner_name().set(owner_name),
        AccountP::bank_id().set(bank_id.clone()),
        AccountP::number().set(number),
        AccountP::created_at().set(created_at),
        AccountP::bank().link(BankP::id().equals(bank_id.clone())),
        vec![],
      )
      .exec()
      .await;

    if !result.id.trim().is_empty() {
      ()
    }
    //Err(String::from("error not add account"));
    Ok(())
  }

  async fn find_account(id: String) -> Result<AccountModel, String> {
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
      Ok(account.into())
    } else {
      Err(String::from("Not found account"))
    };
    account
  }
}
