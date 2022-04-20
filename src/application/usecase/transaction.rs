use crate::domain::model::pix_key::PixKeyRepositoryInterface;
use crate::domain::model::transaction::TransactionActions;
use crate::domain::model::transaction::TransactionDto;
use crate::domain::model::transaction::TransactionModel;
use crate::domain::model::transaction::TransactionRepositoryInterface;
use crate::infrastructure::db::schema::transaction;
use crate::infrastructure::repository::pix::PixkeyRepositoryDb;
use crate::infrastructure::repository::transaction::TransactionRepoDb;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use std::error::Error;
pub struct TransactionUseCase {}

impl TransactionUseCase {
  pub fn register(
    conn: &PgConnection,
    account_id: String,
    amount: u64,
    pix_key_to: String,
    pix_key_kind_to: String,
    description: String,
    id: Option<String>,
  ) -> Result<TransactionModel, Box<dyn Error>> {
    //find account
    let account =
      <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_account(conn, &account_id)?;
    //find key by kind
    let pix_key = <PixkeyRepositoryDb as PixKeyRepositoryInterface>::find_key_by_kind(
      conn,
      pix_key_kind_to,
      pix_key_to,
    )?;
    //check accounts
    let pix_key_account_id = pix_key.account_id;
    if (pix_key_account_id == account_id) {
      panic!("the account from and account to must be different");
    }
    //new transaction and save
    let new_transaction = TransactionDto::new(
      id,
      BigDecimal::from(amount),
      description,
      account.id,
      pix_key.id,
    )?;
    let transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::save(conn, new_transaction)?;
    Ok(transaction)
  }

  pub fn confirm(conn: &PgConnection, transaction_id: String) -> QueryResult<TransactionModel> {
    //find transaction
    let mut find_transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(conn, transaction_id)?;
    // change status
    find_transaction.confirm();
    // update registry
    let result: TransactionModel = diesel::update(transaction::table)
      .set(&find_transaction)
      .get_result(conn)?;
    print!("{:?}", result);
    Ok(result)
  }

  pub fn complete(conn: &PgConnection, transaction_id: String) -> QueryResult<TransactionModel> {
    //find transaction
    let mut find_transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(conn, transaction_id)?;
    // change status
    find_transaction.complete();
    // update registry
    let result: TransactionModel = diesel::update(transaction::table)
      .set(&find_transaction)
      .get_result(conn)?;
    print!("{:?}", result);
    Ok(result)
  }

  pub fn error(
    conn: &PgConnection,
    transaction_id: String,
    reason: String,
  ) -> QueryResult<TransactionModel> {
    //find transaction
    let mut find_transaction =
      <TransactionRepoDb as TransactionRepositoryInterface>::find_by_id(conn, transaction_id)?;
    // change status
    find_transaction.cancel(reason);
    // update registry
    let result: TransactionModel = diesel::update(transaction::table)
      .set(&find_transaction)
      .get_result(conn)?;
    print!("{:?}", result);
    Ok(result)
  }
}
