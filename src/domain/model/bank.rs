use chrono::Utc;
use uuid::Uuid;
struct Bank {
  id: String,
  created_at: String,
  updated_at: String,
  code: String,
  name: String,
  // Accounts []*Account `valid:"_"`
}

impl Bank {
  fn new(code: String, name: String) -> Bank {
    let bank = Bank {
      id: Uuid::new_v4().to_string(),
      code,
      name,
      created_at: Utc::now().to_string(),
      updated_at: Utc::now().to_string(),
    };

    bank
  }
}
