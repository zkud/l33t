use std::io;
use std::process;

fn main() {
  let stdin = io::stdin();
  let mut stdin_lock = stdin.lock();
  let mut stdout = io::stdout();
  if let Err(error) = l33t::run(&mut stdin_lock, &mut stdout) {
    eprintln!("Failed to parse given text: {}", error);
    process::exit(1);
  }
}
