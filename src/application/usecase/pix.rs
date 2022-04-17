use crate::{
  domain::{
    self,
    model::{account::AccountModel, pix_key::PixKeyModel},
  },
  infrastructure::repository::pix::PixkeyRepositoryDb,
};
use domain::model::pix_key::PixKeyRepositoryInterface;

pub struct PixUseCase {}

impl PixUseCase {
  pub async fn register_key(
    kind: String,
    key: String,
    account_id: String,
  ) -> Result<PixKeyModel, String> {
    let account =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(account_id.clone())
        .await
        .unwrap();
    //
    let id = account.id;
    match account_id == id {
      true => {
        let pix_key =
          <PixkeyRepositoryDb as PixKeyRepositoryInterface>::register_key(key, kind, account_id)
            .await;
        pix_key
      }
      false => Err(String::from("Not found account in usecase")),
    }
  }
  pub async fn find_account(id: String) -> Result<AccountModel, String> {
    let account = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(id).await;
    account
  }

  pub async fn find_key(kind: String, key: String) -> Result<PixKeyModel, String> {
    let pix_key =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_key_by_kind(kind, key).await;
    pix_key
  }
}
