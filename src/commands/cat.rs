use std::fs;
use std::path::Path;

pub fn cat(filename: &str) {
  let filepath = Path::new(filename);

  if filepath.is_file() {
    match fs::read_to_string(filename) {
      Ok(c) => println!("{}", c),
      Err(e) => println!("{}", e),
    };
  } else {
    println!("no such file {}", filename);
  }
}
