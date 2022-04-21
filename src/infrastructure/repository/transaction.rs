use crate::api_error::ApiError;
#[allow(dead_code)]
use crate::domain::model::transaction::{TransactionDto, TransactionRepositoryInterface};
use crate::infrastructure::db::schema::transaction;
use crate::{domain::model::transaction::TransactionModel, infrastructure::db::connection};
use diesel::prelude::*;
use std::error::Error;

pub struct TransactionRepoDb {}

impl TransactionRepositoryInterface for TransactionRepoDb {
  fn register(transaction: TransactionDto) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  fn save(transaction: TransactionDto) -> Result<TransactionModel, ApiError> {
    //connection
    let conn = connection()?;
    let transaction = diesel::insert_into(transaction::table)
      .values(transaction)
      .get_result(&conn)?;
    Ok(transaction)
  }

  fn find_by_id(id: String) -> Result<TransactionModel, ApiError> {
    //connection
    let conn = connection()?;
    let find_transaction = transaction::table
      .filter(transaction::id.eq(id))
      .get_result(&conn)?;
    Ok(find_transaction)
  }
}
