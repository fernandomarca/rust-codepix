use crate::api_error::ApiError;
use crate::domain::model::account::AccountModel;
#[allow(dead_code)]
use crate::domain::model::transaction::{TransactionDto, TransactionRepositoryInterface};
use crate::infrastructure::db::schema::{account, transaction};
use crate::{domain::model::transaction::TransactionModel, infrastructure::db::connection};
use diesel::prelude::*;
use log::debug;
use std::error::Error;

pub struct TransactionRepoDb {}

impl TransactionRepoDb {
  pub fn new() -> Self {
    TransactionRepoDb {}
  }
}

impl TransactionRepositoryInterface for TransactionRepoDb {
  fn register(&self, transaction: TransactionDto) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  fn save(&self, transaction: TransactionDto) -> Result<TransactionModel, ApiError> {
    //connection
    let conn = connection()?;
    let transaction = diesel::insert_into(transaction::table)
      .values(transaction)
      .get_result(&conn)?;
    Ok(transaction)
  }

  fn find_by_id(&self, id: String) -> Result<TransactionModel, ApiError> {
    //connection
    let conn = connection()?;
    let find_transaction: TransactionModel = transaction::table
      .filter(transaction::id.eq(id))
      .get_result(&conn)?;
    let account: AccountModel = account::table
      .filter(account::id.eq(String::from("27baa390-dcef-456c-b92e-78611283930f")))
      .get_result(&conn)?;
    //let data: Vec<(TransactionModel, AccountModel)> =
    //transaction::table.inner_join(account::table).load(&conn)?;
    let data: Vec<TransactionModel> = TransactionModel::belonging_to(&account).load(&conn)?;
    // let satellites = SatelliteEntity::belonging_to(&planets)
    // .load(conn)?
    // .grouped_by(&planets);
    debug!("inner_join {:?}", data);
    Ok(find_transaction)
  }
}
