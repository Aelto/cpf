
use std::io;

pub fn prompt() -> String {
  let mut answer = String::new();

  match io::stdin().read_line(&mut answer) {
    Ok(_) => answer,
    Err(err) => {
      println!("error: {}", err);

      String::new()
    }
  }
}
