pub mod domain;
use domain::model::{account::Account, bank::Bank};
fn main() {
    let bank = Bank::new("b01".to_string(), "bank01".to_string());
    match &bank {
        Ok(bank) => println!("{:?}", bank),
        Err(err) => println!("{:?}", err),
    }

    let account = Account::new(
        &bank.unwrap(),
        "3838-12806-8".to_string(),
        "Fernando".to_string(),
    );

    match &account {
        Ok(account) => println!("{:?}", account),
        Err(err) => println!("{:?}", err),
    }
}
