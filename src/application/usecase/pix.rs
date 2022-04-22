use crate::{
  api_error::ApiError,
  domain::{
    self,
    model::{account::AccountModel, pix_key::PixKeyModel},
  },
  infrastructure::repository::pix::PixkeyRepositoryDb,
};
use domain::model::pix_key::PixKeyRepositoryInterface;

pub struct PixUseCase {
  pix_key_repo: PixkeyRepositoryDb,
}

impl PixUseCase {
  pub fn new() -> PixUseCase {
    let db = PixkeyRepositoryDb {};
    PixUseCase { pix_key_repo: db }
  }
  pub fn register_key(
    kind: String,
    key: String,
    account_id: String,
  ) -> Result<PixKeyModel, ApiError> {
    // find account
    let account = PixkeyRepositoryDb::find_account(&account_id)?;
    //register pix
    let result =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::register_key(key, kind, account.id);
    result
  }
  pub fn find_account(id: String) -> Result<AccountModel, ApiError> {
    let account = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(&id);
    account
  }

  pub fn find_key(kind: String, key: String) -> Result<PixKeyModel, ApiError> {
    let pix_key = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_key_by_kind(kind, key);
    pix_key
  }
}
