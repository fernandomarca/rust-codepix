use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
  let uuid = Uuid::parse_str(uuid);
  match uuid {
    Ok(_) => Ok(()),
    _ => Err(ValidationError::new("uuid is invalid")),
  }
}
#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Bank {
  #[validate(length(min = 1), custom = "validate_uuid")]
  pub id: String,

  #[validate(length(min = 1))]
  #[serde(rename = "createdAt")]
  pub created_at: String,

  #[validate(length(min = 1))]
  #[serde(rename = "updatedAt")]
  pub updated_at: String,

  #[validate(length(min = 1))]
  pub code: String,

  #[validate(length(min = 1))]
  pub name: String,
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
