use std::fs;
use std::path::Path;
use std::io;

fn copy_into_dir(origin_path: &Path, destination_path: &Path) -> io::Result<()> {
  let origin_content = fs::read_to_string(origin_path)?;
  let full_destination_path = destination_path.join(origin_path);

  fs::write(full_destination_path, origin_content)
}

fn copy_into_file(origin_path: &Path, destination_path: &Path) -> io::Result<()> {
  if destination_path.is_file() {
    // prompt asking for confirmation
    println!(
      "file {} already exists, do you want to overwrite it? [y/N]",
      destination_path.to_str().unwrap()
    );

    let answer = super::super::prompt::prompt();
    if !answer.starts_with("y") {
      return Ok(());
    }
  }

  let origin_content = fs::read_to_string(origin_path)?;

  fs::write(destination_path, origin_content)
}

pub fn copy(origin: &str, destination: &str) {
  let origin_path = Path::new(origin);

  if !origin_path.is_file() {
    println!("no such file {}", origin);

    return;
  }

  let destination_path = Path::new(destination);

  let action = if destination_path.is_dir() {
    copy_into_dir
  } else {
    copy_into_file
  };

  match action(origin_path, destination_path) {
    Ok(_) => println!(
      "copied {} to {}",
      origin,
      destination_path.to_str().unwrap()
    ),
    Err(e) => println!("error during copy from {} to {}: {}", 
      origin,
      destination_path.to_str().unwrap(),
      e
    ),
  };
}