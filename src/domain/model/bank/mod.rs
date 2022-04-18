mod bank_test;
// use std::marker::PhantomData;
use super::account::AccountModel;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct BankWrapper {
  bank: BankModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankModel {
  pub id: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
  pub code: String,
  pub name: String,
  pub accounts: Vec<AccountModel>,
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
