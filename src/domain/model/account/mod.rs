mod account_test;

// use serde::Deserialize;
// use uuid::Uuid;
// use validator::{Validate, ValidationErrors};

// use crate::infrastructure::prisma_db::db::{AccountP, AccountPData, BankP, PixKeyP};

// #[derive(Debug, Validate, Deserialize, Clone)]
// pub struct Account {
//   pub id: String,

//   #[validate(length(min = 1))]
//   #[serde(rename = "createdAt")]
//   pub created_at: String,

//   #[validate(length(min = 1))]
//   #[serde(rename = "updatedAt")]
//   pub updated_at: String,

//   #[serde(rename = "OwnerName")]
//   #[validate(length(min = 1))]
//   pub owner_name: String,

//   pub bank: Option<BankP>,

//   #[serde(rename = "BankID")]
//   #[validate(length(min = 1))]
//   pub bank_id: String,

//   #[serde(rename = "Number")]
//   #[validate(length(min = 1))]
//   pub number: String,

//   #[serde(rename = "PixKeys")]
//   pub pix_keys: Vec<PixKeyP>,
// }

// impl Account {
//   pub fn new(
//     bank: Bank,
//     number: String,
//     owner_name: String,
//   ) -> Result<AccountPData, ValidationErrors> {
//     let account = AccountPData {
//       id: Uuid::new_v4().to_string(),
//       owner_name,
//       bank: Some(bank.clone()),
//       bank_id: bank.base.id.clone(),
//       number,
//       // pix_keys: Vec::new(),
//       created_at: "".to_string(),
//       updated_at: "".to_string(),
//       bank_id_2: "".to_string(),
//     };
//     account.account_is_valid()?;
//     Ok(account)
//   }

//   fn account_is_valid(&self) -> Result<(), ValidationErrors> {
//     self.validate()
//   }
// }
