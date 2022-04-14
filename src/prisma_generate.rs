use std::process::Command;

fn main() {
  let output = Command::new("npx")
    .arg("prisma")
    .arg("generate")
    .status()
    .expect("npx error");
  println!("status: {}", output);
}
