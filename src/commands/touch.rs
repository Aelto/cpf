use std::path::Path;
use std::fs;

pub fn touch(filename: &str) {
  let filepath = Path::new(filename);

  if !filepath.exists() {
    match fs::write(filename, "") {
      Ok(_) => println!("file {} created", filename),
      Err(e) => println!("{}", e),
    };
  } else {
    println!("file already exists");
  }
}