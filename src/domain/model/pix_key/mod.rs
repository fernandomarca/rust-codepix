mod pix_key_test;

use async_trait::async_trait;
use std::error::Error;

use crate::infrastructure::prisma_db::db::{AccountP, AccountPData, BankP, PixKeyP, PixKeyPData};
#[async_trait]
pub trait PixKeyRepositoryInterface {
  async fn register_key(key: String, kind: String, account_id: String) -> PixKeyPData;
  async fn find_key_by_kind(key: String, kind: String) -> Result<PixKeyP, Box<dyn Error>>;
  async fn add_bank(bank: &BankP) -> Result<(), Box<dyn Error>>;
  async fn add_account(account: &AccountP) -> Result<(), Box<dyn Error>>;
  async fn find_account(id: String) -> AccountPData;
}
