use crate::{
  domain::{
    self,
    model::{account::AccountModel, pix_key::PixKeyModel},
  },
  infrastructure::repository::pix::PixkeyRepositoryDb,
};
use diesel::{result::Error::NotFound, PgConnection, QueryResult};
use domain::model::pix_key::PixKeyRepositoryInterface;
use log::error;

pub struct PixUseCase {}

impl PixUseCase {
  pub fn register_key(
    conn: &PgConnection,
    kind: String,
    key: String,
    account_id: String,
  ) -> QueryResult<PixKeyModel> {
    // find account
    let account =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(conn, &account_id)?;
    //register pix
    let result =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::register_key(conn, key, kind, account_id);
    result
  }
  pub fn find_account(conn: &PgConnection, id: String) -> QueryResult<AccountModel> {
    let account = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(conn, &id);
    match account {
      Ok(account) => Ok(account),
      Err(e) => {
        error!("There was an error while getting a account {}: {}", &id, e);
        match e {
          NotFound => Err(e),
          _ => Err(e),
        }
      }
    }
  }

  pub fn find_key(conn: &PgConnection, kind: String, key: String) -> QueryResult<PixKeyModel> {
    let pix_key =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_key_by_kind(conn, kind, key);
    pix_key
  }
}
