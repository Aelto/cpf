extern crate clap;
extern crate termcolor;

use clap::{App, Arg, SubCommand};

mod commands;
mod prompt;

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
      Some(v) => commands::touch::touch(v),
      None => println!("{}", matches.usage()),
    }
  } else if let Some(matches) = matches.subcommand_matches("cat") {
    match matches.value_of("filename") {
      Some(v) => commands::cat::cat(v),
      None => println!("{}", matches.usage()),
    }
  } else if let Some(matches) = matches.subcommand_matches("cp") {
    if !matches.is_present("origin") || !matches.is_present("destination") {
      println!("{}", matches.usage());

      return;
    }

    let origin = matches.value_of("origin").unwrap();
    let destination = matches.value_of("destination").unwrap();

    commands::copy::copy(origin, destination);
  } else {
    commands::list::list();
  }
}
