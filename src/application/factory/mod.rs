use super::usecase::transaction::TransactionUseCase;
use crate::application::usecase::pix::PixUseCase;
use crate::infrastructure::{
  db::DbConnection, repository::pix::PixkeyRepositoryDb, repository::transaction::TransactionRepoDb,
};

pub fn transaction_usecase_factory(database: &DbConnection) -> TransactionUseCase {
  let pix_repository = PixkeyRepositoryDb::new();
  let transaction_repository = TransactionRepoDb::new();

  let transaction_usecase = TransactionUseCase::new(pix_repository, transaction_repository);

  transaction_usecase
}

pub fn pixkey_usecase_factory(database: &DbConnection) -> PixUseCase {
  let pix_repository = PixkeyRepositoryDb::new();

  let pixkey_usecase = PixUseCase::new(pix_repository);

  pixkey_usecase
}
