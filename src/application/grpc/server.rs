// use tonic::{transport::Server, Request, Response, Status};

// mod pixkey {
//   include!("pb/pixkey.rs");
// }

// use pixgrpc::MyPix;
// use pixkey::;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//   let addr = "[::0]:50051".parse()?;
//   let mypix = MyPix::default();

//   Server::builder()
//     .add_service(GreeterServer::new(greeter))
//     .serve(addr)
//     .await?;

//   // let proto_path = env::var("PROTOS_DIR");
//   // let proto_buffs = env::var("PROTOBUFFS");
//   // print!("{:?},{:?}", proto_buffs, proto_path);
//   Ok(())
// }
