mod application;
use application::usecase::pix::PixUseCase;
mod domain;
mod infrastructure;

fn main() {
  let result = PixUseCase::find_account("1".to_string());
  let pix_key = PixUseCase::register_key(
    "fernando@marca.com".to_string(),
    "email".to_string(),
    "12".to_string(),
  );
  print!("{:?}", pix_key);
}
