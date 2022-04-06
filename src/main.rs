pub mod domain;
use domain::model::{account::*, bank::Bank};
fn main() {
    let bank = Bank::new("b01".to_string(), "bank01".to_string());
    match bank {
        Ok(bank) => println!("{:?}", bank),
        Err(err) => println!("{:?}", err),
    }
}
