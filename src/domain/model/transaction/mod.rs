mod transaction_test;
use crate::domain::model::pix_key::PixKeyModel;
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
  async fn save(transaction: TransactionModel) -> Result<(), Box<dyn Error>>;
  async fn find_by_id(id: String) -> Result<TransactionModel, Box<dyn Error>>;
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
  pub updated_at: Option<DateTime<Utc>>,
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
      updated_at: None,
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

  /// Set the transaction model's created at.
  pub fn set_created_at(&mut self, created_at: DateTime<Utc>) {
    self.created_at = created_at;
  }
}

//   pub fn complete(&mut self) -> Result<(), &'static str> {
//     self.status = TransactionStatus::TransactionCompleted;
//     self.base.updated_at = Utc::now().to_string();
//     let result = self.transaction_is_valid();
//     result
//   }

//   pub fn confirm(&mut self) -> Result<(), &'static str> {
//     self.status = TransactionStatus::TransactionConfirmed;
//     self.base.updated_at = Utc::now().to_string();
//     let result = self.transaction_is_valid();
//     result
//   }

//   pub fn cancel(&mut self, description: String) -> Result<(), &'static str> {
//     self.status = TransactionStatus::TransactionError;
//     self.cancel_description = Some(description);
//     self.base.updated_at = Utc::now().to_string();
//     let result = self.transaction_is_valid();
//     result
//   }
// }
