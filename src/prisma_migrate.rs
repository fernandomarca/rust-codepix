use std::process::Command;

fn main() {
  let output = Command::new("npx")
    .env("DISABLE_ERD", "true")
    .args(["prisma", "migrate", "dev"])
    .status()
    .expect("npx error");
  println!("status: {}", output);
  let mut cpx = Command::new("npm")
    .args(["run", "prisma:db"])
    .status()
    .expect("CP");
  println!("status: {:?}", cpx);
}
