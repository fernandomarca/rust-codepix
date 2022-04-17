pub mod conversion;
mod pix_key_test;
use crate::{
  domain::model::account::AccountModel,
  infrastructure::prisma_db::db::{AccountP, AccountPData},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::bank::BankModel;

#[async_trait]
pub trait PixKeyRepositoryInterface {
  async fn register_key(
    key: String,
    kind: String,
    account_id: String,
  ) -> Result<PixKeyModel, String>;
  async fn find_key_by_kind(kind: String, key: String) -> Result<PixKeyModel, String>;
  async fn add_bank(bank: BankModel) -> Result<(), Box<dyn Error>>;
  async fn add_account(account: AccountModel) -> Result<(), Box<dyn Error>>;
  async fn find_account(id: String) -> Result<AccountModel, String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PixKeyModel {
  pub id: String,
  pub kind: String,
  pub key: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  //pub account: Option<AccountPData>,
  pub account_id: String,
  pub status: String,
}
