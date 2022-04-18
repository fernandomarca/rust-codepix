use crate::domain::model::pix_key::PixKeyRepositoryInterface;
use crate::domain::model::transaction::TransactionActions;
use crate::domain::model::transaction::TransactionDto;
use crate::domain::model::transaction::TransactionRepositoryInterface;
use crate::infrastructure::repository::pix::PixkeyRepositoryDb;
use crate::infrastructure::repository::transaction::TransactionRepoDb;
use chrono::Utc;
pub struct TransactionUseCase {}

impl TransactionUseCase {
  pub fn register(
    account_id: String,
    amount: i64,
    pix_key_to: String,
    pix_key_kind_to: String,
    description: String,
    id: Option<String>,
  ) -> Result<TransactionDto, String> {
    //find account
    //find key by kind
    //new transaction and save

    // _ = <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone()).await?;
    // Ok(transaction)
    todo!()
  }

  pub fn confirm(transaction_id: String) -> Result<TransactionDto, String> {
    // let mut transaction =
    //   <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id).await?;
    // //
    // _ = <TransactionDto as TransactionActions>::confirm(&mut transaction);
    // //
    // _ = <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone());

    // Ok(transaction)
    todo!()
  }

  #[tokio::main]
  pub async fn complete(transaction_id: String) -> Result<TransactionDto, String> {
    // let mut transaction =
    //   <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id).await?;
    // //
    // _ = <TransactionDto as TransactionActions>::complete(&mut transaction);
    // //
    // _ = <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone());

    // Ok(transaction)
    todo!()
  }

  #[tokio::main]
  pub async fn error(transaction_id: String, reason: String) -> Result<TransactionDto, String> {
    // let mut transaction =
    //   <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id).await?;
    // //
    // _ = <TransactionDto as TransactionActions>::cancel(&mut transaction, reason);
    // //
    // _ = <TransactionRepoDb as TransactionRepositoryInterface>::save(transaction.clone());
    // Ok(transaction)
    todo!()
  }
}
