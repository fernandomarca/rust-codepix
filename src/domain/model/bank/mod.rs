mod bank_test;

use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use super::{account::Account, base::Base};

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Bank {
  #[serde(rename = "Base")]
  #[validate]
  pub base: Base,

  #[serde(rename = "Code")]
  #[validate(length(min = 1))]
  pub code: String,

  #[serde(rename = "Name")]
  #[validate(length(min = 1))]
  pub name: String,

  #[serde(rename = "Accounts")]
  #[validate]
  accounts: Vec<Account>,
}

impl Bank {
  pub fn new(code: String, name: String) -> Result<Bank, ValidationErrors> {
    let bank = Bank {
      base: Base::new(),
      code,
      name,
      accounts: vec![],
    };
    bank.is_valid()?;
    Ok(bank)
  }

  fn is_valid(&self) -> Result<(), ValidationErrors> {
    self.validate()
  }
}
