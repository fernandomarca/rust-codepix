mod bank_test;
// use std::marker::PhantomData;
use super::account::AccountModel;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankModel {
  pub id: String,

  created_at: DateTime<Utc>,
  updated_at: Option<DateTime<Utc>>,

  #[serde(rename = "Code")]
  pub code: String,

  #[serde(rename = "Name")]
  pub name: String,

  #[serde(rename = "Accounts")]
  accounts: Vec<AccountModel>,
  // _marker: PhantomData<&'a ()>,
}

impl BankModel {
  pub fn new(code: String, name: String) -> BankModel {
    let bank = BankModel {
      id: Uuid::new_v4().to_string(),
      code,
      name,
      accounts: vec![],
      created_at: Utc::now(),
      updated_at: None,
    };
    bank
  }
}
