mod account_test;

use crate::application::grpc::pb::pixkey::Account;
use crate::domain::model::bank::BankModel;
use crate::domain::model::pix_key::PixKeyModel;
use crate::infrastructure::prisma_db::db::AccountPData;
use crate::infrastructure::prisma_db::db::BankP;
use crate::infrastructure::prisma_db::db::PrismaClient;
use chrono::DateTime;
use chrono::Timelike;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountModel {
  pub id: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
  pub owner_name: String,
  pub bank_id: String,
  pub number: String,
  pub pix_keys: Vec<PixKeyModel>,
}

impl AccountModel {
  pub fn new(bank_id: String, number: String, owner_name: String) -> AccountModel {
    let account = AccountModel {
      id: Uuid::new_v4().to_string(),
      owner_name,
      bank_id,
      number,
      pix_keys: Vec::new(),
      created_at: Utc::now(),
      updated_at: None,
    };
    account
  }
}

//pub(crate)
pub struct AccountWrapper {
  account: AccountModel,
}

//conversion PrismaData response for PixKeyModel
impl From<&AccountPData> for AccountModel {
  fn from(entity: &AccountPData) -> AccountModel {
    let acc = entity;
    AccountModel {
      id: acc.clone().id,
      created_at: acc.created_at.parse::<DateTime<Utc>>().unwrap(),
      updated_at: Some(
        acc
          .updated_at
          .clone()
          .unwrap_or(acc.created_at.parse::<DateTime<Utc>>().unwrap().to_string())
          .parse::<DateTime<Utc>>()
          .unwrap(),
      ),
      owner_name: acc.clone().owner_name,
      bank_id: acc.clone().bank_id,
      number: acc.clone().number,
      //Option<Vec<PixKeyPData>>,
      pix_keys: acc
        .pix_keys()
        .unwrap()
        .into_iter()
        .map(|pk| pk.into())
        .collect(),
    }
  }
}

//conversion PrismaData response for PixKeyModel
impl From<AccountPData> for AccountModel {
  fn from(entity: AccountPData) -> AccountModel {
    let acc = entity;
    AccountModel {
      id: acc.clone().id,
      created_at: acc
        .created_at
        .parse::<DateTime<Utc>>()
        .unwrap_or(Utc::now()),
      updated_at: Some(
        acc
          .clone()
          .updated_at
          .unwrap_or(Utc::now().to_string())
          .parse::<DateTime<Utc>>()
          .unwrap(),
      ),
      owner_name: acc.clone().owner_name,
      bank_id: acc.clone().bank_id,
      number: acc.clone().number,
      //Option<Vec<PixKeyPData>>,
      pix_keys: acc
        .clone()
        .pix_keys()
        .unwrap()
        .into_iter()
        .map(|pk| pk.into())
        .collect(),
    }
  }
}

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
