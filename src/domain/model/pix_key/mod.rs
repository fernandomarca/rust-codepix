pub mod conversion;
mod pix_key_test;
use super::{
  account::{AccountModel, NewAccount},
  bank::NewBank,
};
use crate::{api_error::ApiError, infrastructure::db::schema::pixkey};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub trait PixKeyRepositoryInterface {
  fn register_key(key: String, kind: String, account_id: String) -> Result<PixKeyModel, ApiError>;
  fn find_key_by_kind(kind: String, key: String) -> Result<PixKeyModel, ApiError>;
  fn add_bank(bank: NewBank) -> Result<(), ApiError>;
  fn add_account(account: NewAccount) -> Result<(), ApiError>;
  fn find_account(id: &String) -> Result<AccountModel, ApiError>;
}

#[derive(Deserialize, Insertable)]
#[table_name = "pixkey"]
pub struct NewPix {
  pub id: String,
  pub key: String,
  pub kind: String,
  pub account_id: String,
  pub status: String,
}

impl NewPix {
  pub fn new(key: String, kind: String, account_id: String) -> NewPix {
    NewPix {
      id: Uuid::new_v4().to_string(),
      key,
      kind,
      account_id,
      status: "active".to_string(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Clone, Associations)]
#[table_name = "pixkey"]
#[belongs_to(AccountModel, foreign_key = "account_id")]
pub struct PixKeyModel {
  pub id: String,
  pub kind: String,
  pub key: String,
  pub status: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub account_id: String,
}
