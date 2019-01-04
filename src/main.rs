extern crate clap;
extern crate termcolor;

use std::fs;
use std::io;
use std::path::Path;

use clap::{App, Arg, SubCommand};

fn prompt() -> String {
  let mut answer = String::new();

  match io::stdin().read_line(&mut answer) {
    Ok(_) => answer,
    Err(err) => {
      println!("error: {}", err);

      String::new()
    }
  }
}

fn touch(filename: &str) {
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

fn cat(filename: &str) {
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

fn copy(origin: &str, destination: &str) {
  println!("{} {}", origin, destination);

  let origin_path = Path::new(origin);

  if origin_path.is_file() {
    let destination_path = Path::new(destination);

    if let Ok(origin_content) = fs::read_to_string(origin) {
      // if 'destination' points to a directory
      // create a new file named like the 'origin' file
      if destination_path.is_dir() {
        if let Some(origin_file_name) = origin_path.file_name().and_then(|name| name.to_str()) {
          let full_destination_path = destination_path.join(origin_file_name);

          match fs::write(full_destination_path, origin_content) {
            Ok(_) => println!(
              "copied {} to {}",
              origin,
              destination_path.join(origin_file_name).to_str().unwrap()
            ),
            Err(e) => println!("error when writing to destination {}", e),
          };
        } else {

        }
      } else {
        if destination_path.is_file() {
          // prompt asking for confirmation
          println!(
            "file {} already exists, do you want to overwrite it? [y/N]",
            destination
          );
          let answer = prompt();
          if !answer.starts_with("y") {
            return;
          }
        }

        match fs::write(destination_path, origin_content) {
          Ok(_) => println!(
            "copied {} to {}",
            origin,
            destination_path.to_str().unwrap()
          ),
          Err(e) => println!("error when writing to destination {}", e),
        };
      }
    } else {
      println!("could not read origin {}", origin);
    }
  } else {
    println!("no such file {}", origin);
  }
}

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

fn list() {
  if let Err(error) = fs::read_dir(Path::new("."))
  .and_then(print_entries) {
    println!("error: {}", error);
  }
}

fn main() {
  let matches = App::new("rsf")
    .version("0.1")
    .author("Aelto <hottou.thibault@gmail.com>")
    .about("rust filesystem utility")
    .subcommand(
      SubCommand::with_name("touch")
        .version("0.1")
        .about("creates an empty file")
        .arg(
          Arg::with_name("filename")
            .index(1)
            .help("the file name")
            .required(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("cat")
        .version("0.1")
        .about("reads a file's content")
        .arg(
          Arg::with_name("filename")
            .index(1)
            .help("the file name")
            .required(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("cp")
        .version("0.1")
        .about("copy a file's content to another file")
        .arg(
          Arg::with_name("origin")
            .index(1)
            .help("the origin path")
            .required(true),
        )
        .arg(
          Arg::with_name("destination")
            .index(2)
            .help("the destination file")
            .required(true),
        ),
    )
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("touch") {
    match matches.value_of("filename") {
      Some(v) => touch(v),
      None => println!("{}", matches.usage()),
    }
  } else if let Some(matches) = matches.subcommand_matches("cat") {
    match matches.value_of("filename") {
      Some(v) => cat(v),
      None => println!("{}", matches.usage()),
    }
  } else if let Some(matches) = matches.subcommand_matches("cp") {
    if !matches.is_present("origin") || !matches.is_present("destination") {
      println!("{}", matches.usage());

      return;
    }

    let origin = matches.value_of("origin").unwrap();
    let destination = matches.value_of("destination").unwrap();

    copy(origin, destination);
  } else {
    list();
  }
}
