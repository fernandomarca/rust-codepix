use crate::api_error::ApiError;
use crate::domain::model::pix_key::PixKeyRepositoryInterface;
use crate::domain::model::transaction::TransactionActions;
use crate::domain::model::transaction::TransactionDto;
use crate::domain::model::transaction::TransactionModel;
use crate::domain::model::transaction::TransactionRepositoryInterface;
use crate::infrastructure::db::connection;
use crate::infrastructure::db::schema::transaction;
use crate::infrastructure::repository::pix::PixkeyRepositoryDb;
use crate::infrastructure::repository::transaction::TransactionRepoDb;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
pub struct TransactionUseCase {}

impl TransactionUseCase {
  pub fn register(
    account_id: String,
    amount: f64,
    pix_key_to: String,
    pix_key_kind_to: String,
    description: String,
    id: Option<String>,
  ) -> Result<TransactionModel, ApiError> {
    //find account
    let account = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(&account_id)?;
    //find key by kind
    let pix_key = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_key_by_kind(
      pix_key_kind_to,
      pix_key_to,
    )?;
    //new transaction and save
    let new_transaction = TransactionDto::new(
      id,
      BigDecimal::from(amount),
      description,
      pix_key,
      account.id,
    )
    .map_err(|e| ApiError::new(400, e.to_string()))?;
    //
    let transaction = <TransactionRepoDb as TransactionRepositoryInterface>::save(new_transaction)?;
    Ok(transaction)
  }

  pub fn confirm(
    conn: &PgConnection,
    transaction_id: String,
  ) -> Result<TransactionModel, ApiError> {
    //find transaction
    let mut find_transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id)?;
    // change status
    find_transaction.confirm();
    // update registry
    let result: TransactionModel = diesel::update(transaction::table)
      .set(&find_transaction)
      .get_result(conn)?;
    print!("{:?}", result);
    Ok(result)
  }

  pub fn complete(transaction_id: String) -> Result<TransactionModel, ApiError> {
    //connection
    let conn = connection()?;
    //find transaction
    let mut find_transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id)?;
    // change status
    find_transaction.complete();
    // update registry
    let result: TransactionModel = diesel::update(transaction::table)
      .set(&find_transaction)
      .get_result(&conn)?;
    print!("{:?}", result);
    Ok(result)
  }

  pub fn error(transaction_id: String, reason: String) -> Result<TransactionModel, ApiError> {
    //connection
    let conn = connection()?;
    //find transaction
    let mut find_transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(transaction_id)?;
    // change status
    find_transaction.cancel(reason);
    // update registry
    let result: TransactionModel = diesel::update(transaction::table)
      .set(&find_transaction)
      .get_result(&conn)?;
    print!("{:?}", result);
    Ok(result)
  }
}
