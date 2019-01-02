extern crate clap;

use std::fs;
use std::path::Path;

use clap::{App, Arg, SubCommand};

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

fn main() {
  let matches = App::new("rsf")
    .version("0.1")
    .author("Aelto <hottou.thibault@gmail.com>")
    .about("rust filesystem utility")
    .subcommand(
      SubCommand::with_name("touch")
        .version("0.1")
        .about("creates an empty file")
        .arg(Arg::with_name("filename").index(1).help("the file name")),
    )
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("touch") {
    match matches.value_of("filename") {
      Some(v) => touch(v),
      None => println!("{}", matches.usage()),
    }
  };
}
