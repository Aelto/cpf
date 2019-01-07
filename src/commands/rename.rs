
use std::fs;
use std::path::Path;

pub fn rename(origin: &str, destination: &str) {
  let origin_path = Path::new(origin);

  if !origin_path.exists() {
    println!("no such file {}", origin);

    return;
  }

  let destination_path = Path::new(destination);

  if destination_path.exists() {
    println!("file or directory with the name {} already exists", destination);

    return;
  }

  match fs::rename(origin_path, destination_path) {
    Ok(_) => println!("renamed {} to {}", origin, destination),
    Err(e) => println!("rename error: {}", e)
  };
}
