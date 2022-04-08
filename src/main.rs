mod db;
pub mod domain;
use db::{PrismaClient, User};
pub use domain::model::{account::Account, bank::Bank, pix_key::PixKey};

#[tokio::main]
async fn main() {
    let client = PrismaClient::new().await;

    // Required values are passed in as separate arguments
    let user = client
        .user()
        .create_one(
            User::id().set("user3".to_string()),
            User::display_name().set("User 0".to_string()),
            // Optional arguments can be added in a Vec as the last parameter
            vec![],
        )
        .exec()
        .await;

    print!("{:?}", user);
    // Find all records that match some parameters
    // let unique_user = client
    //     .user()
    //     .find_unique(User::id().equals("0001".to_string()))
    //     .exec()
    //     .await;

    // print!("{:?}", unique_user);

    // let bank = Bank::new("b01".to_string(), "bank01".to_string());
    // match &bank {
    //     Ok(bank) => println!("{:?}", bank),
    //     Err(err) => println!("{:?}", err),
    // }

    // let account = Account::new(
    //     bank.unwrap(),
    //     "3838-12806-8".to_string(),
    //     "Fernando".to_string(),
    // );

    // match &account {
    //     Ok(account) => println!("{:?}", account),
    //     Err(err) => println!("{:?}", err),
    // }

    // let pix_key = PixKey::new(
    //     "email".to_string(),
    //     account.unwrap(),
    //     "3838-12806-8".to_string(),
    // );

    // match &pix_key {
    //     Ok(pix_key) => println!("{:?}", pix_key),
    //     Err(err) => println!("{:?}", err),
    // }
}
