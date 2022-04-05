use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};
#[derive(Debug, Validate, Deserialize)]
pub struct Bank {
  // #[validate(uuid = "lower")]
  #[validate(length(min = 1))]
  id: String,

  #[validate(length(min = 1))]
  #[serde(rename = "createdAt")]
  created_at: String,

  #[validate(length(min = 1))]
  #[serde(rename = "updatedAt")]
  updated_at: String,

  #[validate(length(min = 1))]
  code: String,
  #[validate(length(min = 1))]
  name: String,
  // Accounts []*Account `valid:"_"`
}

impl Bank {
  pub fn new(code: String, name: String) -> Result<Bank, ValidationErrors> {
    let bank = Bank {
      id: Uuid::new_v4().to_string(),
      code,
      name,
      created_at: Utc::now().to_string(),
      updated_at: Utc::now().to_string(),
    };
    bank.is_valid()?;
    Ok(bank)
  }

  fn is_valid(&self) -> Result<(), ValidationErrors> {
    self.validate()
  }
}
