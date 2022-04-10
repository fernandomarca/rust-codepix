use crate::domain::model::pix_key::PixKeyRepositoryInterface;
use crate::domain::model::transaction::TransactionActions;
use crate::domain::model::transaction::TransactionDto;
use crate::domain::model::transaction::TransactionRepositoryInterface;
use crate::infrastructure::repository::pix::PixkeyRepositoryDb;
use crate::infrastructure::repository::transaction::TransactionRepoDb;
use chrono::Utc;
pub struct TransactionUseCase {}

impl TransactionUseCase {
  #[tokio::main]
  pub async fn register(
    account_id: String,
    amount: i64,
    pix_key_to: String,
    pix_key_kind_to: String,
    description: String,
    id: Option<String>,
  ) -> Result<TransactionDto, String> {
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
    let transaction = TransactionDto {
      id,
      account_from_id: account.unwrap().id,
      amount,
      pix_key_id_to: pix_key.unwrap().id,
      description,
      status: "pending".to_string(),
      created_at: Utc::now(),
      updated_at: Utc::now(),
    };

    _ = <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone()).await?;
    Ok(transaction)
  }

  #[tokio::main]
  pub async fn confirm(transaction_id: String) -> Result<TransactionDto, String> {
    let mut transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id).await?;
    //
    _ = <TransactionDto as TransactionActions>::confirm(&mut transaction);
    //
    _ = <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone());

    Ok(transaction)
  }

  #[tokio::main]
  pub async fn complete(transaction_id: String) -> Result<TransactionDto, String> {
    let mut transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id).await?;
    //
    _ = <TransactionDto as TransactionActions>::complete(&mut transaction);
    //
    _ = <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone());

    Ok(transaction)
  }

  #[tokio::main]
  pub async fn error(transaction_id: String, reason: String) -> Result<TransactionDto, String> {
    let mut transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id).await?;
    //
    _ = <TransactionDto as TransactionActions>::cancel(&mut transaction, reason);
    //
    _ = <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone());
    Ok(transaction)
  }
}
