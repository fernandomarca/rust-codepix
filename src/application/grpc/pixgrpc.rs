#[allow(dead_code)]
use crate::api_error::ApiErrorGrpc;
use crate::application::factory::pixkey_usecase_factory;
use crate::application::usecase::pix::PixUseCase;
use crate::infrastructure::db::connection;
pub mod pixkey {
  include!("pb/pixkey.rs");
}
use self::pixkey::pix_service_server::PixServiceServer;
use self::pixkey::{PixKeyCreateRequest, PixKeyCreatedResult, PixKeyFindRequest, PixKeyResponse};

use log::{debug, error, info};
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
    let account_id: String = req.account_id.clone().into();
    //
    let database = connection();
    let database = match database {
      Ok(database) => Ok(database),
      Err(e) => Err(ApiErrorGrpc::new(e)),
    }?;
    let pix_usecase = pixkey_usecase_factory(&database);
    let result = pix_usecase.register_key(kind, key.clone(), account_id);
    //
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
      Err(error) => {
        error!("There was an error registering PixKey {}: {}", &key, error);
        Err(ApiErrorGrpc::new(error))
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
    //
    let database = connection();
    let database = match database {
      Ok(database) => Ok(database),
      Err(e) => Err(ApiErrorGrpc::new(e)),
    }?;
    let pix_usecase = pixkey_usecase_factory(&database);
    //
    let pixkey = pix_usecase.find_key(key.clone());

    match pixkey {
      Ok(r) => {
        let grpc = r.into();
        Ok(Response::new(grpc))
      }
      Err(error) => {
        error!("There was an error fetching PixKey {}: {}", &key, error);
        Err(ApiErrorGrpc::new(error))
      }
    }
  }
}

pub async fn server_grpc(
  pix_service: MyPix,
  port: String,
) -> Result<(), Box<dyn std::error::Error>> {
  let addr = format!("[::0]:{}", port).parse()?;

  // creating a service
  info!("Server listening on {}", addr);

  Server::builder()
    .add_service(PixServiceServer::new(pix_service))
    .serve(addr)
    .await?;
  Ok(())
}
