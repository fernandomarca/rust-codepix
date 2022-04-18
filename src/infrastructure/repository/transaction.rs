use crate::domain::model;
use crate::domain::model::transaction::TransactionModel;
#[allow(dead_code)]
use crate::infrastructure::repository::transaction::model::transaction::TransactionDto;
use async_trait::async_trait;
use model::transaction::TransactionRepositoryInterface;
use std::error::Error;

pub struct TransactionRepoDb {}

#[async_trait]
impl TransactionRepositoryInterface for TransactionRepoDb {
  async fn register(_transaction: TransactionModel) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  async fn save(transaction: TransactionDto) -> Result<(), String> {
    todo!()
  }

  async fn find_by_id(id: String) -> Result<TransactionDto, String> {
    todo!()
  }
}
