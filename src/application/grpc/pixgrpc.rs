#[allow(dead_code)]
use crate::application::usecase::pix::PixUseCase;
pub mod pixkey {
  include!("pb/pixkey.rs");
}
use self::pixkey::pix_service_server::PixServiceServer;
use self::pixkey::{Account, PixKeyPData, PixKeyRegistration};
use pixkey::pix_service_server::PixService;
use pixkey::PixKey;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyPix {}

#[tonic::async_trait]
impl PixService for MyPix {
  //register pixkey
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
  //find pixkey
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

pub async fn server_grpc(pix_service: MyPix) -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::0]:50051".parse()?;

  // creating a service
  println!("Server listening on {}", addr);

  Server::builder()
    .add_service(PixServiceServer::new(pix_service))
    .serve(addr)
    .await?;
  Ok(())
}
