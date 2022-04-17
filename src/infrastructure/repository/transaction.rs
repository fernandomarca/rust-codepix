use crate::domain::model;
use crate::domain::model::transaction::TransactionModel;
use crate::infrastructure::prisma_db::db::{AccountP, PixKeyP, PrismaClient, TransactionP};
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
    let prisma_client = PrismaClient::new().await;
    //
    //
    let TransactionDto {
      id: _,
      account_from_id,
      amount,
      pix_key_id_to,
      status,
      description,
      created_at,
      updated_at: _,
    } = transaction;
    let new_transaction = prisma_client
      .transaction_p()
      .create_one(
        TransactionP::account_from().link(AccountP::id().equals(account_from_id.clone())),
        TransactionP::account_from_id().set(account_from_id),
        TransactionP::amount().set(amount as f32),
        TransactionP::pix_key_to().link(PixKeyP::id().equals(pix_key_id_to.clone())),
        TransactionP::pix_key_id_to().set(pix_key_id_to),
        TransactionP::status().set(status),
        TransactionP::description().set(description),
        TransactionP::created_at().set(created_at.to_string()),
        vec![],
      )
      .exec()
      .await;
    //
    let result = match &new_transaction.id.trim().is_empty() {
      true => Err(String::from("error save transaction")),
      false => Ok(()),
    };
    result
  }

  async fn find_by_id(id: String) -> Result<TransactionDto, String> {
    let prisma_client = PrismaClient::new().await;
    let transaction = prisma_client
      .transaction_p()
      .find_unique(TransactionP::id().equals(id.clone()))
      .exec()
      .await;
    let transaction_dto: TransactionDto = transaction.clone().into();
    match transaction.id == id {
      true => Ok(transaction_dto),
      false => Err(String::from("not found transaction")),
    }
  }
}
