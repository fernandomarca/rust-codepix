use bigdecimal::{BigDecimal, ToPrimitive};
use log::warn;
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::model::transaction::{TransactionDto, TransactionModel};

#[derive(Serialize, Deserialize, Validate)]
struct TransactionApp {
  #[validate(length(min = 1))]
  id: String,
  #[validate(length(min = 1))]
  account_id: String,
  #[validate(range(min = 1))]
  amount: f64,
  #[validate(length(min = 1))]
  pix_key_to: String,
  // #[validate(length(min = 1))]
  // pix_key_kind_to: String,
  #[validate(length(min = 1))]
  description: String,
  #[validate(length(min = 1))]
  status: String,
  error: Option<String>,
}

impl From<TransactionApp> for TransactionDto {
  fn from(transaction: TransactionApp) -> Self {
    TransactionDto {
      id: transaction.id,
      amount: BigDecimal::from(transaction.amount),
      status: transaction.status,
      description: transaction.description,
      account_from_id: transaction.account_id,
      pix_key_id_to: transaction.pix_key_to,
    }
  }
}

pub fn parse_json(msg: BorrowedMessage) -> TransactionDto {
  let payload = msg.payload_view::<str>();
  let payload = match payload {
    Some(Ok(p)) => p,
    None => "",
    Some(Err(e)) => {
      warn!("Error while deserializing message payload: {:?}", e);
      ""
    }
  };
  let t_app: TransactionApp = serde_json::from_str(payload).expect("error transaction_parse_json");
  let transaction_dto = t_app.into();
  // need make refactory in function of validate the transaction
  // transaction_dto.transaction_is_valid(account_id_in_pix_key: String);
  transaction_dto
}

impl From<&TransactionModel> for TransactionApp {
  fn from(transaction: &TransactionModel) -> Self {
    TransactionApp {
      id: transaction.id.clone(),
      amount: ToPrimitive::to_f64(&transaction.amount).unwrap(),
      status: transaction.status.clone(),
      description: transaction.description.clone(),
      account_id: transaction.account_from_id.clone(),
      pix_key_to: transaction.pix_key_id_to.clone(),
      error: None,
    }
  }
}

pub fn to_json(transaction: &TransactionModel) -> String {
  let result: TransactionApp = transaction.into();
  let transaction_json = serde_json::to_string(&result).expect("error transaction_to_json");
  transaction_json
}
