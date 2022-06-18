use super::random::maybe_translate;

pub fn leetefy_char(chr: &char) -> String {
  if !chr.is_ascii() {
    return chr.to_string();
  }

  let possibility = 0.2;

  let chr = chr.to_string();
  let chr = chr.as_str();
  match chr {
    "a" => maybe_translate(chr, vec!["/\\", "@"], possibility),
    "b" => maybe_translate(chr, vec!["I3"], possibility),
    "c" => maybe_translate(chr, vec!["[", "("], possibility),
    "d" => maybe_translate(chr, vec!["|)", "T)"], possibility),
    "e" => maybe_translate(chr, vec!["3"], possibility),
    "f" => maybe_translate(chr, vec!["|=", "/="], possibility),
    "g" => maybe_translate(chr, vec!["[."], possibility),
    "h" => maybe_translate(chr, vec!["|-|", "|~|"], possibility),
    "i" => maybe_translate(chr, vec!["1"], possibility),
    "j" => maybe_translate(chr, vec!["_|"], possibility),
    "k" => maybe_translate(chr, vec!["|<"], possibility),
    "l" => maybe_translate(chr, vec!["|_", "7"], possibility),
    "m" => maybe_translate(chr, vec!["/\\/\\", "[V]"], possibility),
    "n" => maybe_translate(chr, vec!["|\\|", "[\\]"], possibility),
    "o" => maybe_translate(chr, vec!["0", "()"], possibility),
    "p" => maybe_translate(chr, vec!["|7", "|*"], possibility),
    "q" => maybe_translate(chr, vec!["0_"], possibility),
    "r" => maybe_translate(chr, vec!["I2", "|`"], possibility),
    "s" => maybe_translate(chr, vec!["5", "$"], possibility),
    "t" => maybe_translate(chr, vec!["7", "+"], possibility),
    "u" => maybe_translate(chr, vec!["|_|"], possibility),
    "v" => maybe_translate(chr, vec!["\\/"], possibility),
    "w" => maybe_translate(chr, vec!["vv", "\\V/"], possibility),
    "x" => maybe_translate(chr, vec!["><", "}{"], possibility),
    "y" => maybe_translate(chr, vec!["\\|/"], possibility),
    "z" => maybe_translate(chr, vec!["2"], possibility),
    _ => chr.to_string(),
  }
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
