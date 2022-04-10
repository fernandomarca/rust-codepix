use crate::domain::model::pix_key::PixKeyRepositoryInterface;
use crate::domain::model::transaction::TransactionModel;
use crate::domain::model::transaction::TransactionRepositoryInterface;
use crate::infrastructure::repository::pix::PixkeyRepositoryDb;
use crate::infrastructure::repository::transaction::TransactionRepoDb;
use chrono::Utc;
struct TransactionUseCase {}

impl TransactionUseCase {
  pub async fn register(
    account_id: String,
    amount: i64,
    pix_key_to: String,
    pix_key_kind_to: String,
    description: String,
    id: Option<String>,
  ) -> Result<TransactionModel, String> {
    //find account
    let account =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(account_id.clone()).await;
    match &account {
      Ok(_) => (),
      Err(e) => Err(e).unwrap(),
    };
    //find key by kind
    let pix_key = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_key_by_kind(
      pix_key_to,
      pix_key_kind_to,
    )
    .await;
    match &pix_key {
      Ok(_) => (),
      Err(e) => Err(e).unwrap(),
    };
    //new transaction and save
    let transaction = TransactionModel {
      id,
      account_from: account.clone().unwrap(),
      account_from_id: account.unwrap().id,
      amount,
      pix_key_to: pix_key.clone().unwrap(),
      pix_key_id_to: pix_key.unwrap().id,
      description,
      status: "pending".to_string(),
      updated_at: None,
      created_at: Utc::now(),
    };
    <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone());
    Ok(transaction)
  }
}
