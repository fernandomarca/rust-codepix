mod bank_test;
use crate::infrastructure::prisma_db::db::BankPData;

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

impl From<&BankPData> for BankModel {
  fn from(entity: &BankPData) -> BankModel {
    let b = entity;
    BankModel {
      id: b.clone().id,
      created_at: b.clone().created_at.parse::<DateTime<Utc>>().unwrap(),
      updated_at: Some(
        b.clone()
          .updated_at
          .unwrap_or(b.created_at.parse::<DateTime<Utc>>().unwrap().to_string())
          .parse::<DateTime<Utc>>()
          .unwrap(),
      ),
      code: b.clone().code,
      name: b.clone().name,
      accounts: b
        .account()
        .unwrap()
        .into_iter()
        .map(|pk| pk.into())
        .collect(),
    }
  }
}
