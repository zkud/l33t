use rand;
use rand::seq::SliceRandom;

pub fn leetefy_line(line: &str) -> String {
  let line: Vec<String> = line.trim_end().chars().map(|c| leetefy_char(&c)).collect();
  line.join("")
}

fn leetefy_char(c: &char) -> String {
  if !c.is_ascii() {
    return c.to_string();
  }

  match c.to_ascii_lowercase() {
    'a' => random_choice(vec!["a", "/\\", "@"]),
    'b' => random_choice(vec!["b", "I3", "@"]),
    'c' => random_choice(vec!["c", "[", "("]),
    'd' => random_choice(vec!["d", "|)", "T)"]),
    'e' => random_choice(vec!["e", "3"]),
    'f' => random_choice(vec!["f", "|=", "/="]),
    'g' => random_choice(vec!["g", "[."]),
    'h' => random_choice(vec!["h", "|-|", "|~|"]),
    'i' => random_choice(vec!["i", "1"]),
    'j' => random_choice(vec!["j", "_|"]),
    'k' => random_choice(vec!["k", "|<"]),
    'l' => random_choice(vec!["l", "|_", "7"]),
    'm' => random_choice(vec!["m", "/\\/\\", "[V]"]),
    'n' => random_choice(vec!["n", "|\\|", "[\\]"]),
    'o' => random_choice(vec!["o", "0", "()"]),
    'p' => random_choice(vec!["p", "|7", "|*"]),
    'q' => random_choice(vec!["q", "0_"]),
    'r' => random_choice(vec!["r", "I2", "|`"]),
    's' => random_choice(vec!["s", "5", "$"]),
    't' => random_choice(vec!["t", "7", "+"]),
    'u' => random_choice(vec!["u", "|_|"]),
    'v' => random_choice(vec!["v", "\\/"]),
    'w' => random_choice(vec!["w", "vv", "\\V/"]),
    'x' => random_choice(vec!["x", "><", "}{"]),
    'y' => random_choice(vec!["y", "\\|/"]),
    'z' => random_choice(vec!["z", "2"]),
    anouther => anouther.to_string(),
  }
}

fn random_choice(variants: Vec<&str>) -> String {
  let mut range = rand::thread_rng();
  variants.choose(&mut range).unwrap().to_string()
}
