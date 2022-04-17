#[allow(dead_code)]
use crate::application::usecase::pix::PixUseCase;
pub mod pixkey {
  include!("pb/pixkey.rs");
}

use self::pixkey::pix_service_server::PixServiceServer;
use self::pixkey::{PixKeyCreateRequest, PixKeyCreatedResult, PixKeyFindRequest, PixKeyResponse};
use log::{debug, error};
use pixkey::pix_service_server::PixService;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
#[derive(Debug, Default)]
pub struct MyPix {}

#[tonic::async_trait]
impl PixService for MyPix {
  //register pixkey
  async fn register_pix_key(
    self: &MyPix,
    request: Request<PixKeyCreateRequest>,
  ) -> Result<Response<PixKeyCreatedResult>, Status> {
    debug!(" chegou uma requisição: {:?}", request);
    let req = request.into_inner();
    let kind: String = req.kind.clone().into();
    let key: String = req.key.clone().into();
    let account_id = req.account_id.clone().into();
    //
    let result = PixUseCase::register_key(kind, key.clone(), account_id).await;

    match result {
      Ok(r) => {
        let pix_response: PixKeyResponse = r.into();

        let grpc = PixKeyCreatedResult {
          id: pix_response.id,
          status: pix_response.status,
          error: "".to_string(),
        };
        Ok(Response::new(grpc))
      }
      Err(e) => {
        error!("There was an error registering PixKey {}: {}", &key, e);
        match e {
          _ => Err(Status::unknown(format!(
            "here was an error registering PixKey {}: {}",
            &key, e
          ))),
        }
      }
    }
  }
  //find pixkey
  async fn find(
    &self,
    request: Request<PixKeyFindRequest>,
  ) -> Result<Response<PixKeyResponse>, Status> {
    debug!("Got a request: {:?}", request);
    let req = request.into_inner();

    let kind: String = req.kind.clone().into();
    let key: String = req.key.clone().into();
    print!("{}, {}", kind, key);
    let pixkey = PixUseCase::find_key(kind, key.clone()).await;

    match pixkey {
      Ok(r) => {
        let grpc = r.into();
        Ok(Response::new(grpc))
      }
      Err(e) => {
        error!("There was an error fetching PixKey {}: {}", &key, e);
        match e {
          _ => Err(Status::unknown(format!(
            "here was an error registering PixKey {}: {}",
            &key, e
          ))),
        }
      }
    }
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
