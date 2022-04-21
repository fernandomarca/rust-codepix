mod account_test;

use crate::domain::model::bank::BankModel;
use crate::infrastructure::db::schema::account;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
  Debug, Deserialize, Serialize, Queryable, Identifiable, Clone, Associations, AsChangeset,
)]
#[table_name = "account"]
#[belongs_to(BankModel, foreign_key = "bank_id")]
// #[belongs_to(PixKeyModel, foreign_key = "pix_keys")]
pub struct AccountModel {
  pub id: String,
  pub owner_name: String,
  pub number: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub bank_id: String,
  pub pix_keys: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "account"]
pub struct NewAccount {
  pub id: String,
  pub owner_name: String,
  pub number: String,
  pub bank_id: String,
}

impl NewAccount {
  pub fn new(owner_name: String, number: String, bank_id: String) -> NewAccount {
    NewAccount {
      id: Uuid::new_v4().to_string(),
      owner_name,
      number,
      bank_id,
    }
  }
}

//pub(crate)
// pub struct AccountWrapper {
//   account: AccountModel,
// }

// impl From<AccountWrapper> for Account {
//   fn from(entity: AccountWrapper) -> Account {
//     let t = prost_types::Timestamp {
//       seconds: Timelike::second(&entity.account.created_at) as i64,
//       nanos: 0,
//     };
//     let acc = entity.account;
//     Account {
//       account_id: acc.id,
//       account_number: acc.number,
//       bank_id: acc.bank_id,
//       bank_name: acc.bank.name,
//       owner_name: acc.owner_name,
//       created_at: Some(t),
//     }
//   }
// }
