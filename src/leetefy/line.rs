use super::char;
use super::suffix;

pub fn leetefy_line(line: &str) -> String {
  let line: Vec<String> = suffix::leetefy_suffixes(line.trim_end())
    .chars()
    .map(|c| char::leetefy_char(&c))
    .collect();
  line.join("")
}
