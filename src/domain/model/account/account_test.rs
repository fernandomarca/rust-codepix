#[allow(unused_variables)]
#[cfg(test)]
mod account_test {
  use uuid::Uuid;
  use validator::ValidationErrors;

  use crate::domain::model::{account::Account, bank::Bank};

  fn factory_bank() -> Result<Bank, ValidationErrors> {
    let bank = Bank::new("12345678".to_string(), "Bank of Rust".to_string());
    bank
  }

  fn account_factory() -> Result<Account, ValidationErrors> {
    let bank = factory_bank().unwrap();
    let account = Account::new(&bank, "3838-12806-8".to_string(), "Fernando".to_string());
    account
  }

  #[test]
  fn test_account_new() {
    let account = account_factory();
    assert_eq!(account.is_ok(), true);
  }
  #[test]
  fn account_with_valid_uuid() {
    let account = account_factory().unwrap();
    assert!(Uuid::parse_str(&account.base.id).is_ok());
  }

  #[test]
  fn required_inputs_not_empty_or_blank() {
    let account = account_factory().unwrap();
    let Account {
      base,
      owner_name,
      bank,
      bank_id,
      number,
    } = account;
    assert_eq!(owner_name.trim().is_empty(), false);
    assert_eq!(bank_id.trim().is_empty(), false);
    assert_eq!(number.trim().is_empty(), false);
  }
}
