fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure()
    .build_server(false)
    .out_dir("src/application/grpc/pb")
    .compile(
      &["src/application/grpc/protofiles/helloworld.proto"],
      &["src/application/grpc/protofiles"],
    )?;

  tonic_build::configure()
    .build_server(false)
    .out_dir("src/application/grpc/pb")
    .compile(
      &["src/application/grpc/protofiles/pixkey.proto"],
      &["src/application/grpc/protofiles"],
    )?;
  // tonic_build::compile_protos("src/application/grpc/protofiles/helloworld.proto")?;

  Ok(())
}
