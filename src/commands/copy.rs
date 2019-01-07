use std::fs;
use std::path::Path;
use std::io;

fn copy_into_dir(origin_path: &Path, destination_path: &Path) -> io::Result<()> {
  let full_destination_path = destination_path.join(origin_path);
  fs::copy(origin_path, full_destination_path)?;
  Ok(())
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
      return Err(io::Error::new(io::ErrorKind::Other, "cancelled"));
    }
  }

  fs::copy(origin_path, destination_path)?;
  Ok(())
}

pub fn copy(origin: &str, destination: &str) {
  let origin_path = Path::new(origin);

  if !origin_path.exists() {
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
    Err(e) => println!("copy error: {}",e),
  };
}