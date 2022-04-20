use crate::domain::model::transaction::TransactionModel;
#[allow(dead_code)]
use crate::domain::model::transaction::{TransactionDto, TransactionRepositoryInterface};
use crate::infrastructure::db::schema::transaction;
use diesel::prelude::*;
use std::error::Error;

pub struct TransactionRepoDb {}

impl TransactionRepositoryInterface for TransactionRepoDb {
  fn register(transaction: TransactionDto) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  fn save(conn: &PgConnection, transaction: TransactionDto) -> QueryResult<TransactionModel> {
    let transaction = diesel::insert_into(transaction::table)
      .values(transaction)
      .get_result(conn)?;
    Ok(transaction)
  }

  fn find_by_id(conn: &PgConnection, id: String) -> QueryResult<TransactionModel> {
    let find_transaction = transaction::table
      .filter(transaction::id.eq(id))
      .get_result(conn)?;
    Ok(find_transaction)
  }
}
