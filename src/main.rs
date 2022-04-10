mod application;
use application::usecase::{pix::PixUseCase, transaction::TransactionUseCase};
mod domain;
mod infrastructure;

fn main() {
  //let account = PixUseCase::find_account("1".to_string());
  // let pix_key = PixUseCase::register_key(
  //   "fernando@marca.com".to_string(),
  //   "email".to_string(),
  //   "1".to_string(),
  // );
  // let find_key = PixUseCase::find_key("fernando@marca.com".to_string(), "email".to_string());
  // let transaction = TransactionUseCase::register(
  //   String::from("1"),
  //   100,
  //   String::from("fernando@marca.com"),
  //   String::from("email"),
  //   String::from("recebimento"),
  //   Some(String::from("1")),
  // );
  let cancel_transaction = TransactionUseCase::error(String::from("1"), String::from("teste"));
  print!("{:?}", cancel_transaction);
}
