fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure()
    .build_server(true)
    .out_dir("src/application/grpc/pb")
    .compile(
      &[
        "src/application/grpc/protofiles/pixkey.proto",
        "src/application/grpc/protofiles/helloworld.proto",
      ],
      &["src/application/grpc/protofiles/"],
    )?;
  Ok(())
}
