use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError};

fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
  let uuid = Uuid::parse_str(uuid);
  match uuid {
    Ok(_) => Ok(()),
    _ => Err(ValidationError::new("uuid is invalid")),
  }
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Base {
  #[validate(
    length(min = 1),
    custom(
      function = "validate_uuid",
      code = "uuid",
      message = "uuid is invalid in base"
    )
  )]
  pub id: String,

  #[validate(length(min = 1))]
  #[serde(rename = "createdAt")]
  pub created_at: String,

  #[validate(length(min = 1))]
  #[serde(rename = "updatedAt")]
  pub updated_at: String,
}

impl Base {
  pub fn new() -> Base {
    Base {
      id: Uuid::new_v4().to_string(),
      created_at: Utc::now().to_string(),
      updated_at: Utc::now().to_string(),
    }
  }
}
