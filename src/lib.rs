mod leetefy;

use std::error;
use std::io;
use std::io::prelude::*;

pub fn run(
  in_buf: &mut impl io::BufRead,
  out_buf: &mut impl io::Write,
) -> Result<(), Box<dyn error::Error>> {
  for line in in_buf.lines() {
    let line = leetefy::leetefy_line(&line?);
    out_buf.write_fmt(format_args!("{}\n", line))?;
  }
  Ok(())
}
