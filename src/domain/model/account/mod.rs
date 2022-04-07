mod account_test;

// use std::marker::PhantomData;

use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use super::{bank::Bank, base::Base, pix_key::PixKey};

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Account<'a> {
  #[serde(rename = "Base")]
  #[validate]
  pub base: Base,

  #[serde(rename = "OwnerName")]
  #[validate(length(min = 1))]
  pub owner_name: String,

  #[validate]
  pub bank: Bank<'a>,

  #[serde(rename = "BankID")]
  #[validate(length(min = 1))]
  pub bank_id: String,

  #[serde(rename = "Number")]
  #[validate(length(min = 1))]
  pub number: String,

  #[serde(rename = "PixKeys")]
  pub pix_keys: Vec<PixKey<'a>>,
  // _marker: PhantomData<&'a ()>,
}

impl Account<'_> {
  pub fn new<'a>(
    bank: Bank<'a>,
    number: String,
    owner_name: String,
  ) -> Result<Account<'a>, ValidationErrors> {
    let account = Account {
      base: Base::new(),
      owner_name,
      bank: bank.clone(),
      bank_id: bank.base.id.clone(),
      number,
      pix_keys: Vec::new(),
      // _marker: PhantomData,
    };
    account.account_is_valid()?;
    Ok(account)
  }

  fn account_is_valid(&self) -> Result<(), ValidationErrors> {
    self.validate()
  }
}
