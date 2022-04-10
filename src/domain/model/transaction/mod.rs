#[allow(dead_code)]
mod transaction_test;
use crate::infrastructure::prisma_db::db::AccountPData;
use crate::infrastructure::prisma_db::db::PixKeyPData;
use crate::infrastructure::prisma_db::db::TransactionPData;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[async_trait]
pub trait TransactionRepositoryInterface {
  async fn register(transaction: TransactionModel) -> Result<(), Box<dyn Error>>;
  async fn save(transaction: TransactionDto) -> Result<(), String>;
  async fn find_by_id(id: String) -> Result<TransactionDto, String>;
}
#[allow(dead_code)]
pub struct Transactions {
  transactions: Vec<TransactionPData>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionModel {
  pub id: Option<String>,
  pub account_from: AccountPData,
  pub account_from_id: String,
  pub amount: i64,

  pub pix_key_to: PixKeyPData,
  pub pix_key_id_to: String,
  pub status: String,
  pub description: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
impl TransactionModel {
  pub fn new(
    &self,
    account_from: AccountPData,
    amount: i64,
    pix_key_to: PixKeyPData,
    description: String,
    id: String,
  ) -> Result<TransactionModel, &'static str> {
    let new_id = if let true = id.trim().is_empty() {
      Uuid::new_v4().to_string()
    } else {
      id
    };

    let transaction = TransactionModel {
      id: Some(new_id),
      account_from: account_from.clone(),
      account_from_id: account_from.id,
      amount,
      pix_key_to: pix_key_to.clone(),
      pix_key_id_to: pix_key_to.id,
      status: "pending".to_string(),
      description,
      created_at: Utc::now(),
      updated_at: Utc::now(),
    };
    self.transaction_is_valid()?;
    Ok(transaction)
  }
  fn transaction_is_valid(&self) -> Result<(), &'static str> {
    if self.amount <= 0 {
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

  // Set the transaction model's created at.
  // pub fn set_created_at(&mut self, created_at: DateTime<Utc>) {
  //   self.created_at = created_at;
  // }
  // pub fn complete(&mut self) -> Result<(), &'static str> {
  //   self.status = String::from("completed");
  //   self.updated_at = Some(Utc::now());
  //   let result = self.transaction_is_valid()?;
  //   Ok(result)
  // }

  // pub fn confirm(&mut self) -> Result<(), &'static str> {
  //   self.status = String::from("confirmed");
  //   self.updated_at = Some(Utc::now());
  //   let result = self.transaction_is_valid()?;
  //   Ok(result)
  // }

  // pub fn cancel(&mut self, description: String) -> Result<(), &'static str> {
  //   self.status = String::from("error");
  //   self.updated_at = Some(Utc::now());
  //   self.description = description;
  //   let result = self.transaction_is_valid()?;
  //   Ok(result)
  // }
}

pub trait TransactionActions {
  fn complete(&mut self);
  fn confirm(&mut self);
  fn cancel(&mut self, description: String);
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionDto {
  pub id: Option<String>,
  pub account_from_id: String,
  pub amount: i64,
  pub pix_key_id_to: String,
  pub status: String,
  pub description: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl TransactionDto {
  /// Set the transaction dto's updated at.
  pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
    self.updated_at = updated_at;
  }

  /// Set the transaction dto's status.
  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }
}

impl TransactionActions for TransactionDto {
  fn complete(&mut self) {
    self.set_status(String::from("completed"));
    self.set_updated_at(Utc::now());
  }

  fn confirm(&mut self) {
    self.set_status(String::from("confirmed"));
    self.set_updated_at(Utc::now());
  }

  fn cancel(&mut self, description: String) {
    self.set_status(String::from("error"));
    self.set_updated_at(Utc::now());
    self.description = description;
  }
}

impl From<TransactionModel> for TransactionDto {
  fn from(t: TransactionModel) -> Self {
    Self {
      id: t.id,
      account_from_id: t.account_from_id,
      amount: t.amount,
      pix_key_id_to: t.pix_key_id_to,
      status: t.status,
      description: t.description,
      created_at: t.created_at,
      updated_at: t.updated_at,
    }
  }
}

impl From<TransactionPData> for TransactionDto {
  fn from(t: TransactionPData) -> Self {
    Self {
      id: Some(t.id),
      account_from_id: t.account_from_id,
      amount: t.amount as i64,
      pix_key_id_to: t.pix_key_id_to,
      status: t.status,
      description: t.description,
      created_at: t.created_at.parse::<DateTime<Utc>>().unwrap(),
      updated_at: t
        .updated_at
        .unwrap_or(Utc::now().to_string())
        .parse::<DateTime<Utc>>()
        .unwrap(),
    }
  }
}
