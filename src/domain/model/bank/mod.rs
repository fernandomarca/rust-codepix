mod bank_test;

use std::marker::PhantomData;

use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use super::{account::Account, base::Base};

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Bank<'a> {
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
  accounts: Vec<Account<'a>>,
  _marker: PhantomData<&'a ()>,
}

impl Bank<'_> {
  pub fn new(code: String, name: String) -> Result<Bank<'static>, ValidationErrors> {
    let bank = Bank {
      base: Base::new(),
      code,
      name,
      accounts: vec![],
      _marker: PhantomData,
    };
    bank.is_valid()?;
    Ok(bank)
  }

  fn is_valid(&self) -> Result<(), ValidationErrors> {
    self.validate()
  }
}
