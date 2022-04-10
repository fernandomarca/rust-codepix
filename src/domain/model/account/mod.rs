mod account_test;

use crate::domain::model::bank::BankModel;
use crate::domain::model::pix_key::PixKeyModel;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountModel {
  pub id: String,

  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,

  #[serde(rename = "updatedAt")]
  pub updated_at: Option<DateTime<Utc>>,

  #[serde(rename = "OwnerName")]
  pub owner_name: String,

  pub bank: BankModel,

  #[serde(rename = "BankID")]
  pub bank_id: String,

  #[serde(rename = "Number")]
  pub number: String,

  #[serde(rename = "PixKeys")]
  pub pix_keys: Vec<PixKeyModel>,
}

impl AccountModel {
  pub fn new(bank: BankModel, number: String, owner_name: String) -> AccountModel {
    let account = AccountModel {
      id: Uuid::new_v4().to_string(),
      owner_name,
      bank: bank.clone(),
      bank_id: bank.id,
      number,
      pix_keys: Vec::new(),
      created_at: Utc::now(),
      updated_at: None,
    };
    account
  }
}
