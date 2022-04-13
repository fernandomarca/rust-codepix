mod application;
mod domain;
mod infrastructure;

use pixkey::pix_service_server::PixServiceServer;
use tonic::transport::Server;
mod pixkey {
  include!("application/grpc/pb/pixkey.rs");
}

#[allow(dead_code)]
use tonic::{Request, Response, Status};

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
    let pixkey = PixUseCase::find_key(kind, key).await;
    let kindp = pixkey.clone().unwrap().kind;
    let keyp = pixkey.clone().unwrap().key;

    let id = pixkey.clone().unwrap().id;
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

async fn server_grpc() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::0]:50051".parse()?;
  let mypix = MyPix::default();

  Server::builder()
    .add_service(PixServiceServer::new(mypix))
    .serve(addr)
    .await?;
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  server_grpc().await;
  Ok(())
}
