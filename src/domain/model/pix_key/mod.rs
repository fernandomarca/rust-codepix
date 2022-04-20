pub mod conversion;
mod pix_key_test;
use super::{
  account::{AccountModel, NewAccount},
  bank::NewBank,
};
use crate::infrastructure::db::schema::pixkey;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
pub trait PixKeyRepositoryInterface {
  fn register_key(
    conn: &PgConnection,
    key: String,
    kind: String,
    account_id: String,
  ) -> QueryResult<PixKeyModel>;
  fn find_key_by_kind(conn: &PgConnection, kind: String, key: String) -> QueryResult<PixKeyModel>;
  fn add_bank(conn: &PgConnection, bank: NewBank) -> Result<(), Box<dyn Error>>;
  fn add_account(conn: &PgConnection, account: NewAccount) -> Result<(), Box<dyn Error>>;
  fn find_account(conn: &PgConnection, id: &String) -> QueryResult<AccountModel>;
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
