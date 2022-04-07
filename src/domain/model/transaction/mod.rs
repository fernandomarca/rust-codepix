mod transaction_test;

use super::account::Account;
use super::base::Base;
use super::pix_key::PixKey;
use std::error::Error;

use chrono::Utc;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum TransactionStatus {
  TransactionPending,
  TransactionCompleted,
  TransactionError,
  TransactionConfirmed,
}

trait TransactionRepositoryInterface {
  fn register(transaction: &Transaction) -> Result<(), Box<dyn Error>>;
  fn save(transaction: &Transaction) -> Result<(), Box<dyn Error>>;
  fn find_by_id(id: String) -> Result<Transaction<'static>, Box<dyn Error>>;
}

#[allow(dead_code)]
pub struct Transactions<'a> {
  transactions: Vec<Transaction<'a>>,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Transaction<'a> {
  #[serde(rename = "Base")]
  #[validate]
  base: Base,

  #[serde(rename = "AccountFrom")]
  #[validate]
  account_from: Account<'a>,

  #[serde(rename = "AccountFromID")]
  #[validate(length(min = 1))]
  account_from_id: String,

  #[serde(rename = "Amount")]
  #[validate(range(min = 1))]
  amount: i64,

  #[serde(rename = "PixKeyTo")]
  #[validate]
  pix_key_to: PixKey<'a>,

  #[serde(rename = "PixKeyIdTo")]
  #[validate(length(min = 1))]
  pix_key_id_to: String,

  #[serde(rename = "Status")]
  pub status: TransactionStatus,

  #[serde(rename = "Description")]
  #[validate(length(min = 1))]
  description: String,

  #[serde(rename = "CancelDescription")]
  #[validate(length(min = 1))]
  cancel_description: Option<String>,
}

impl Transaction<'_> {
  pub fn new<'a>(
    account_from: &'a Account,
    amount: i64,
    pix_key_to: &'a PixKey,
    description: String,
  ) -> Result<Transaction<'a>, Box<dyn Error>> {
    let transaction = Transaction {
      base: Base::new(),
      account_from: account_from.clone(),
      account_from_id: account_from.base.id.clone(),
      amount,
      pix_key_to: pix_key_to.clone(),
      pix_key_id_to: pix_key_to.base.id.clone(),
      status: TransactionStatus::TransactionPending,
      description,
      cancel_description: None,
    };
    transaction.transaction_is_valid()?;
    Ok(transaction)
  }

  fn transaction_is_valid(&self) -> Result<(), &'static str> {
    let result = self.validate();

    match result {
      Ok(_) => {
        if self.amount <= 0 {
          return Err("the amount must be greater than 0");
        }
        if self.status != TransactionStatus::TransactionPending
          && self.status != TransactionStatus::TransactionCompleted
          && self.status != TransactionStatus::TransactionError
          && self.status != TransactionStatus::TransactionConfirmed
        {
          return Err("the status must be pending, completed, confirmed or error");
        }
        if self.account_from_id == self.pix_key_to.account_id {
          return Err("the account from and account to must be different");
        }
        Ok(())
      }
      Err(_) => Err("transaction is not valid"),
    }
  }

  pub fn complete(&mut self) -> Result<(), &'static str> {
    self.status = TransactionStatus::TransactionCompleted;
    self.base.updated_at = Utc::now().to_string();
    let result = self.transaction_is_valid();
    result
  }

  pub fn confirm(&mut self) -> Result<(), &'static str> {
    self.status = TransactionStatus::TransactionConfirmed;
    self.base.updated_at = Utc::now().to_string();
    let result = self.transaction_is_valid();
    result
  }

  pub fn cancel(&mut self, description: String) -> Result<(), &'static str> {
    self.status = TransactionStatus::TransactionError;
    self.cancel_description = Some(description);
    self.base.updated_at = Utc::now().to_string();
    let result = self.transaction_is_valid();
    result
  }
}
