use rand;
use rand::seq::SliceRandom;

pub fn leetefy_char(chr: &char) -> String {
  if !chr.is_ascii() {
    return chr.to_string();
  }

  let chr = chr.to_string();
  match chr.as_str() {
    "a" => random_choice(vec![&chr, "/\\", "@"]),
    "b" => random_choice(vec![&chr, "I3"]),
    "c" => random_choice(vec![&chr, "[", "("]),
    "d" => random_choice(vec![&chr, "|)", "T)"]),
    "e" => random_choice(vec![&chr, "3"]),
    "f" => random_choice(vec![&chr, "|=", "/="]),
    "g" => random_choice(vec![&chr, "[."]),
    "h" => random_choice(vec![&chr, "|-|", "|~|"]),
    "i" => random_choice(vec![&chr, "1"]),
    "j" => random_choice(vec![&chr, "_|"]),
    "k" => random_choice(vec![&chr, "|<"]),
    "l" => random_choice(vec![&chr, "|_", "7"]),
    "m" => random_choice(vec![&chr, "/\\/\\", "[V]"]),
    "n" => random_choice(vec![&chr, "|\\|", "[\\]"]),
    "o" => random_choice(vec![&chr, "0", "()"]),
    "p" => random_choice(vec![&chr, "|7", "|*"]),
    "q" => random_choice(vec![&chr, "0_"]),
    "r" => random_choice(vec![&chr, "I2", "|`"]),
    "s" => random_choice(vec![&chr, "5", "$"]),
    "t" => random_choice(vec![&chr, "7", "+"]),
    "u" => random_choice(vec![&chr, "|_|"]),
    "v" => random_choice(vec![&chr, "\\/"]),
    "w" => random_choice(vec![&chr, "vv", "\\V/"]),
    "x" => random_choice(vec![&chr, "><", "}{"]),
    "y" => random_choice(vec![&chr, "\\|/"]),
    "z" => random_choice(vec![&chr, "2"]),
    _ => chr,
  }
}

fn random_choice(variants: Vec<&str>) -> String {
  let mut range = rand::thread_rng();
  variants.choose(&mut range).unwrap().to_string()
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_ignores_non_askii_chars() {
    let non_askii = 'э';
    let leet = super::leetefy_char(&non_askii);
    assert_eq!(non_askii.to_string(), leet);
  }

  #[test]
  fn it_ignores_numbers() {
    let number = '1';
    let leet = super::leetefy_char(&number);
    assert_eq!(number.to_string(), leet);
  }

  #[test]
  fn it_translates_askii_alphabet_to_leet() {
    let askii_a_code = 97u8;
    let askii_z_code = 97u8;

    for symbol in askii_a_code..=askii_z_code {
      let symbol = symbol as char;
      let leet = super::leetefy_char(&symbol);
      assert!(is_valid_leet(&symbol, &leet));
    }
  }

  fn is_valid_leet(symbol: &char, translation: &str) -> bool {
    match symbol {
      'a' => vec!["a", "/\\", "@"].contains(&translation),
      'b' => vec!["b", "I3"].contains(&translation),
      'c' => vec!["c", "[", "("].contains(&translation),
      'd' => vec!["d", "|)", "T)"].contains(&translation),
      'e' => vec!["e", "3"].contains(&translation),
      'f' => vec!["f", "|=", "/="].contains(&translation),
      'g' => vec!["g", "[."].contains(&translation),
      'h' => vec!["h", "|-|", "|~|"].contains(&translation),
      'i' => vec!["i", "1"].contains(&translation),
      'j' => vec!["j", "_|"].contains(&translation),
      'k' => vec!["k", "|<"].contains(&translation),
      'l' => vec!["l", "|_", "7"].contains(&translation),
      'm' => vec!["m", "/\\/\\", "[V]"].contains(&translation),
      'n' => vec!["n", "|\\|", "[\\]"].contains(&translation),
      'o' => vec!["o", "0", "()"].contains(&translation),
      'p' => vec!["p", "|7", "|*"].contains(&translation),
      'q' => vec!["q", "0_"].contains(&translation),
      'r' => vec!["r", "I2", "|`"].contains(&translation),
      's' => vec!["s", "5", "$"].contains(&translation),
      't' => vec!["t", "7", "+"].contains(&translation),
      'u' => vec!["u", "|_|"].contains(&translation),
      'v' => vec!["v", "\\/"].contains(&translation),
      'w' => vec!["w", "vv", "\\V/"].contains(&translation),
      'x' => vec!["x", "><", "}{"].contains(&translation),
      'y' => vec!["y", "\\|/"].contains(&translation),
      'z' => vec!["z", "2"].contains(&translation),
      _ => false,
    }
  }
}
