use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure()
    .build_server(true)
    .out_dir("src/application/grpc/pb")
    .compile(
      &["src/application/grpc/protofiles/pixkey.proto"],
      &["src/application/grpc/protofiles/"],
    )?;

  //exec prisma migrate and copy dev.db for root ./
  let output = Command::new("npx")
    .env("DISABLE_ERD", "true")
    .args(["prisma", "migrate", "dev"])
    .status()
    .expect("npx prisma migrate error");
  println!("status: {}", output);
  println!("cargo:rerun-if-changed=prisma/schema.prisma");
  let mut cpx = Command::new("npm")
    .args(["run", "prisma:db"])
    .status()
    .expect("copy dev.db error");
  println!("status: {:?}", cpx);
  println!("cargo:rerun-if-changed=prisma/schema.prisma");
  Ok(())
}
