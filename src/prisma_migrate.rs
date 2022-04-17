use std::process::Command;

fn main() {
  let cpx = Command::new("npm")
    .args(["run", "prisma:migrate"])
    .status()
    .expect("error prisma migrate");
  println!("status: {:?}", cpx);
}
