use crate::application::usecase::transaction;
use crate::domain::model;
use crate::domain::model::transaction::TransactionModel;
use crate::infrastructure::prisma_db::db::{
  AccountP, PixKeyP, PixKeyPData, PrismaClient, TransactionP, TransactionPData,
};
use async_trait::async_trait;
use model::transaction::TransactionRepositoryInterface;
use std::error::Error;

pub struct TransactionRepoDb {}

#[async_trait]
impl TransactionRepositoryInterface for TransactionRepoDb {
  async fn register(transaction: TransactionModel) -> Result<(), Box<dyn Error>> {
    todo!()
  }

  async fn save(transaction: TransactionModel) -> Result<(), Box<dyn Error>> {
    let prisma_client = PrismaClient::new().await;
    let TransactionModel {
      id,
      account_from,
      account_from_id,
      amount,
      pix_key_to,
      pix_key_id_to,
      status,
      description,
      created_at,
      updated_at,
    } = transaction;
    prisma_client.transaction_p().create_one(
      TransactionP::id().set(id.unwrap()),
      TransactionP::account_from().link(AccountP::id().equals(account_from_id.clone())),
      TransactionP::account_from_id().set(account_from_id),
      TransactionP::amount().set(amount as f32),
      TransactionP::pix_key_to().link(PixKeyP::id().equals(pix_key_id_to.clone())),
      TransactionP::pix_key_id_to().set(pix_key_id_to),
      TransactionP::status().set(status),
      TransactionP::description().set(description),
      TransactionP::created_at().set(created_at.to_string()),
      vec![],
    );
    Ok(())
  }

  async fn find_by_id(id: String) -> Result<TransactionModel, Box<dyn Error>> {
    todo!()
  }
}
