use std::env;

use helloworld::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};
mod helloworld {
  include!("pb/helloworld.rs");
}
mod pixkey {
  include!("pb/pixkey.rs");
}
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
  async fn say_hello(
    &self,
    request: Request<HelloRequest>, // Accept request of type HelloRequest
  ) -> Result<Response<HelloReply>, Status> {
    // Return an instance of type HelloReply
    println!("Got a request: {:?}", request);

    let reply = helloworld::HelloReply {
      message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
    };

    Ok(Response::new(reply)) // Send back our formatted greeting
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;
  let greeter = MyGreeter::default();

  Server::builder()
    .add_service(GreeterServer::new(greeter))
    .serve(addr)
    .await?;

  // let proto_path = env::var("PROTOS_DIR");
  // let proto_buffs = env::var("PROTOBUFFS");
  // print!("{:?},{:?}", proto_buffs, proto_path);
  Ok(())
}
