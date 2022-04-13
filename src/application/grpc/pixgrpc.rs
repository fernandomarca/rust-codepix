#[allow(dead_code)]
use tonic::{Request, Response, Status};

mod pixkey {
  include!("pb/pixkey.rs");
}

use pixkey::pix_service_server::PixService;
use pixkey::PixKey;

use self::pixkey::{Account, PixKeyPData, PixKeyRegistration};
use crate::application::usecase::pix::PixUseCase;

#[derive(Debug, Default)]
pub struct MyPix {}

#[tonic::async_trait]
impl PixService for MyPix {
  async fn register_pix_key(
    self: &MyPix,
    request: Request<PixKeyRegistration>,
  ) -> Result<Response<PixKeyPData>, Status> {
    println!("Got a request: {:?}", request);

    let account = Account {
      account_id: "1".to_string(),
      account_number: "213".to_string(),
      bank_id: "1".to_string(),
      bank_name: "fer".to_string(),
      owner_name: "12".to_string(),
      created_at: "create".to_string(),
    };

    let result = PixKeyPData {
      id: "st".to_string(),
      kind: "st".to_string(),
      key: "st".to_string(),
      account: Some(account),
      created_at: "create".to_string(),
    };
    Ok(Response::new(result))
  }
  async fn find(&self, request: Request<PixKey>) -> Result<Response<PixKeyPData>, Status> {
    println!("Got a request: {:?}", request);

    let req = request.into_inner();

    let kind: String = req.kind.clone().into();
    let key: String = req.key.clone().into();
    print!("{}, {}", kind, key);
    let pixkey = PixUseCase::find_key(key, kind).await.unwrap();
    let kindp = pixkey.kind;
    let keyp = pixkey.key;

    let id = pixkey.id;
    let account = Account {
      account_id: "1".to_string(),
      account_number: "213".to_string(),
      bank_id: "1".to_string(),
      bank_name: "fer".to_string(),
      owner_name: "12".to_string(),
      created_at: "create".to_string(),
    };

    let result = PixKeyPData {
      id,
      kind: kindp,
      key: keyp,
      account: Some(account),
      created_at: "create".to_string(),
    };

    Ok(Response::new(result))
  }
}
