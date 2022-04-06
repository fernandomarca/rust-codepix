mod account_test;

use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use super::{bank::Bank, base::Base};

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Account {
  #[serde(rename = "Base")]
  #[validate]
  pub base: Base,

  #[serde(rename = "OwnerName")]
  #[validate(length(min = 1))]
  pub owner_name: String,

  #[validate]
  pub bank: Bank,

  #[serde(rename = "BankID")]
  #[validate(length(min = 1))]
  pub bank_id: String,

  #[serde(rename = "Number")]
  #[validate(length(min = 1))]
  pub number: String,
  // pix_key: Vec<PixKey>,
}

impl Account {
  pub fn new(bank: &Bank, number: String, owner_name: String) -> Result<Account, ValidationErrors> {
    let account = Account {
      base: Base::new(),
      owner_name,
      bank: bank.clone(),
      bank_id: bank.base.id.clone(),
      number,
    };
    account.account_is_valid()?;
    Ok(account)
  }

  fn account_is_valid(&self) -> Result<(), ValidationErrors> {
    self.validate()
  }
}
