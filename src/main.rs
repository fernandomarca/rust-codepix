use std::error::Error;

mod application;
mod domain;
mod infrastructure;

#[path = "application/grpc/pixgrpc.rs"]
mod api_grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let pix_service = api_grpc::MyPix {};
  api_grpc::server_grpc(pix_service).await;

  // print!("{:?}", env);
  Ok(())
}
