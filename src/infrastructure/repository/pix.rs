use crate::domain::model::{
  account::AccountModel,
  bank::BankModel,
  pix_key::{NewPix, PixKeyModel, PixKeyRepositoryInterface},
};
use crate::infrastructure::db::schema::{account, pixkey};
use async_trait::async_trait;
use diesel::{PgConnection, PgTextExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use std::error::Error;
pub struct PixkeyRepositoryDb {}

#[async_trait]
impl PixKeyRepositoryInterface for PixkeyRepositoryDb {
  fn register_key(
    conn: &PgConnection,
    key: String,
    kind: String,
    account_id: String,
  ) -> QueryResult<PixKeyModel> {
    //register Pixkey
    let new_pix = NewPix::new(key, kind, account_id);
    let pixkey: PixKeyModel = diesel::insert_into(pixkey::table)
      .values(&new_pix)
      .get_result(conn)
      .expect("Error register pixkey");
    Ok(pixkey)
  }

  fn find_key_by_kind(kind: String, key: String) -> Result<PixKeyModel, String> {
    todo!()
  }

  fn add_bank(bank: BankModel) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  fn add_account(account: AccountModel) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  fn find_account(conn: &PgConnection, id: &String) -> QueryResult<AccountModel> {
    let account: AccountModel = account::table
      .filter(account::id.ilike(id))
      .first(conn)
      .expect("Error find account");
    Ok(account)
  }
}
