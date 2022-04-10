use crate::{
  domain,
  infrastructure::{
    prisma_db::db::{AccountPData, PixKeyPData},
    repository::pix::PixkeyRepositoryDb,
  },
};
use domain::model::pix_key::PixKeyRepositoryInterface;

pub struct PixUseCase {}

impl PixUseCase {
  #[tokio::main]
  pub async fn register_key(
    key: String,
    kind: String,
    account_id: String,
  ) -> Result<PixKeyPData, String> {
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
        Ok(pix_key)
      }
      false => Err(String::from("Not found account in usecase")),
    }
  }
  #[tokio::main]
  pub async fn find_account(id: String) -> Result<AccountPData, String> {
    let account = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(id).await;
    account
  }

  pub async fn find_key(key: String, kind: String) -> Result<PixKeyPData, String> {
    let pix_key =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_key_by_kind(key, kind).await;
    pix_key
  }
}
