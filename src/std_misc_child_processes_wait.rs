use std::process::Command;

pub fn main() {
  let _process = Command::new("sleep").arg("5").spawn();

  println!("reached end of main");
}
