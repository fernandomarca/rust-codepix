use serde::Deserialize;
use validator::Validate;

use super::{bank::Bank, base::Base};

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Account {
  #[serde(rename = "Base")]
  #[validate]
  base: Base,

  #[serde(rename = "OwnerName")]
  #[validate(length(min = 1))]
  pub owner_name: String,

  #[validate]
  pub bank: Bank,

  #[serde(rename = "BankID")]
  #[validate(length(min = 1))]
  bank_id: String,

  #[serde(rename = "Number")]
  #[validate(length(min = 1))]
  number: String,
  // pix_key: Vec<PixKey>,
}

impl Account {
  pub fn new(bank: &Bank, number: String, owner_name: String) -> Account {
    Account {
      base: Base::new(),
      owner_name,
      bank: bank.clone(),
      bank_id: bank.base.id.clone(),
      number,
    }
  }
}
