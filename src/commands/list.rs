
use std::fs;
use std::io;
use std::path::Path;

fn print_entry(entry: fs::DirEntry) -> io::Result<()> {
  use std::io::Write;
  use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

  if entry.path().is_dir() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(&mut stdout, "{} ", entry.file_name().into_string().unwrap_or(String::new()))?;
    stdout.reset()?;
  } else {
    print!(
      "{} ",
      entry.file_name().into_string().unwrap_or(String::new())
    );
  }

  Ok(())
}

fn print_entries(it: fs::ReadDir) -> io::Result<()> {
  it.map(|entry| entry.and_then(print_entry)).collect()
}

pub fn list() {
  if let Err(error) = fs::read_dir(Path::new("."))
  .and_then(print_entries) {
    println!("error: {}", error);
  }
}
