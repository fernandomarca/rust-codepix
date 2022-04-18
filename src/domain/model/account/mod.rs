mod account_test;

use crate::infrastructure::db::schema::account;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Clone)]
#[table_name = "account"]
pub struct AccountModel {
  pub id: String,
  pub owner_name: String,
  pub number: String,
  //pub bank_id: String,
  //pub pix_keys: Vec<PixKeyModel>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

// impl AccountModel {
//   pub fn new(bank_id: String, number: String, owner_name: String) -> AccountModel {
//     let account = AccountModel {
//       id: Uuid::new_v4().to_string(),
//       owner_name,
//       bank_id,
//       number,
//       pix_keys: Vec::new(),
//       created_at: Utc::now(),
//       updated_at: None,
//     };
//     account
//   }
// }

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
