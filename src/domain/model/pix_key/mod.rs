mod pix_key_test;

use super::{account::Account, bank::Bank, base::Base};
use std::error::Error;

use serde::Deserialize;
use validator::Validate;

pub trait PixKeyRepositoryInterface {
  fn register_key(&self) -> Result<PixKey, Box<dyn Error>>;
  fn find_key_by_kind(key: String, kind: String) -> Result<PixKey<'static>, Box<dyn Error>>;
  fn add_bank(bank: &Bank) -> Result<(), Box<dyn Error>>;
  fn add_account(account: &Account) -> Result<(), Box<dyn Error>>;
  fn find_account(id: String) -> Result<Account<'static>, Box<dyn Error>>;
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct PixKey<'a> {
  #[serde(rename = "Base")]
  #[validate]
  pub base: Base,

  #[serde(rename = "Kind")]
  #[validate(length(min = 1))]
  pub kind: String,

  #[serde(rename = "Key")]
  #[validate(length(min = 1))]
  pub key: String,

  #[serde(rename = "AccountID")]
  #[validate(length(min = 1))]
  pub account_id: String,

  #[serde(rename = "Account")]
  pub account: Account<'a>,

  #[serde(rename = "Status")]
  #[validate(length(min = 1))]
  pub status: String,
}

impl PixKey<'_> {
  pub fn new<'a>(
    kind: String,
    account: Account<'a>,
    key: String,
  ) -> Result<PixKey<'a>, Box<dyn Error>> {
    let pix_key = PixKey {
      base: Base::new(),
      kind,
      key,
      account_id: account.base.id.clone(),
      account: account.clone(),
      status: "active".to_string(),
    };
    pix_key.pix_key_is_valid()?;
    account.pix_keys.unwrap().push(pix_key.clone());
    Ok(pix_key)
  }
  fn pix_key_is_valid(&self) -> Result<(), &'static str> {
    let result = self.validate();
    if result.is_ok() {
      if (self.kind != "email".to_string()) && (self.kind != "cpf".to_string()) {
        return Err("invalid type of key");
      }
      if (self.status != "active".to_string()) && (self.status != "inactive".to_string()) {
        return Err("invalid status");
      }
      Ok(())
    } else {
      Err("pix_key is not valid:")
    }
  }
}
