mod pix_key_test;

use crate::domain::model::account::AccountModel;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::infrastructure::prisma_db::db::{AccountP, AccountPData, BankP, PixKeyPData};
#[async_trait]
pub trait PixKeyRepositoryInterface {
  async fn register_key(key: String, kind: String, account_id: String) -> PixKeyPData;
  async fn find_key_by_kind(kind: String, key: String) -> Result<PixKeyPData, String>;
  async fn add_bank(bank: &BankP) -> Result<(), Box<dyn Error>>;
  async fn add_account(account: &AccountP) -> Result<(), Box<dyn Error>>;
  async fn find_account(id: String) -> Result<AccountPData, String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PixKeyModel {
  pub id: String,
  kind: String,
  key: String,
  created_at: String,
  updated_at: Option<String>,
  account: AccountModel,
  account_id: String,
  status: String,
}
