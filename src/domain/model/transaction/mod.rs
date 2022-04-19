#[allow(dead_code)]
mod transaction_test;
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

use super::account::AccountModel;
use super::pix_key::PixKeyModel;

#[async_trait]
pub trait TransactionRepositoryInterface {
  async fn register(transaction: TransactionModel) -> Result<(), Box<dyn Error>>;
  async fn save(transaction: TransactionDto) -> Result<(), String>;
  async fn find_by_id(id: String) -> Result<TransactionDto, String>;
}
#[allow(dead_code)]
pub struct Transactions {
  transactions: Vec<TransactionModel>,
}

use crate::infrastructure::db::schema::transaction;
#[derive(Debug, Queryable, Identifiable, Clone, Associations)]
#[table_name = "transaction"]
#[belongs_to(AccountModel, foreign_key = "account_from_id")]
#[belongs_to(PixKeyModel, foreign_key = "pix_key_id_to")]
pub struct TransactionModel {
  pub id: String,
  pub amount: BigDecimal,
  pub status: String,
  pub description: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  // relations
  pub account_from: AccountModel,
  pub account_from_id: String,
  // relations
  pub pix_key_to: PixKeyModel,
  pub pix_key_id_to: String,
}
pub trait TransactionActions {
  fn complete(&mut self);
  fn confirm(&mut self);
  fn cancel(&mut self, description: String);
}

#[derive(Debug, Insertable)]
#[table_name = "transaction"]
pub struct TransactionDto {
  pub id: String,
  pub amount: BigDecimal,
  pub status: String,
  pub description: String,
  pub account_from_id: String,
  pub pix_key_id_to: String,
}

impl TransactionDto {
  pub fn new(
    &self,
    id: String,
    amount: BigDecimal,
    description: String,
    account_from_id: String,
    pix_key_id_to: String,
  ) -> Result<TransactionDto, &'static str> {
    let verify_id = if let true = id.trim().is_empty() {
      Uuid::new_v4().to_string()
    } else {
      id
    };
    let transaction = TransactionDto {
      id: verify_id,
      amount,
      status: "pending".to_string(),
      description,
      account_from_id,
      pix_key_id_to,
    };
    self.transaction_is_valid()?;
    Ok(transaction)
  }
  /// Set the transaction status.
  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  //valid transaction?
  fn transaction_is_valid(&self) -> Result<(), &'static str> {
    if self.amount <= BigDecimal::from(0) {
      return Err("the amount must be greater than 0");
    }
    if self.status != "pending".to_string()
      && self.status != "completed".to_string()
      && self.status != "error".to_string()
      && self.status != "confirmed".to_string()
    {
      return Err("the status must be pending, completed, confirmed or error");
    }
    if self.account_from_id == self.pix_key_id_to {
      return Err("the account from and account to must be different");
    }
    Ok(())
  }
}

impl TransactionActions for TransactionDto {
  fn complete(&mut self) {
    self.set_status(String::from("completed"));
  }

  fn confirm(&mut self) {
    self.set_status(String::from("confirmed"));
  }

  fn cancel(&mut self, description: String) {
    self.set_status(String::from("error"));
    self.description = description;
  }
}
