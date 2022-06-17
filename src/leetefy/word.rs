pub fn leetefy_word(word: &str) -> String {
  match word {
    "own" | "gain" => String::from("pwn"),
    "owned" | "gained" => String::from("pwn'd"),
    "owning" | "gaining" => String::from("pwng"),
    "soft" | "software" => String::from("warez"),
    "hacker" => String::from("hackxor"),
    "newbie" => String::from("noob"),
    "porn" | "pornography" => String::from("pron"),
    _ => String::from(word),
  }
}

#[cfg(test)]
mod tests {
  use super::leetefy_word;

  #[test]
  fn it_translates_to_dialect() {
    let translations = [
      ("own", "pwn"),
      ("gain", "pwn"),
      ("owned", "pwn'd"),
      ("gained", "pwn'd"),
      ("soft", "warez"),
      ("software", "warez"),
      ("hacker", "hackxor"),
      ("newbie", "noob"),
      ("porn", "pron"),
      ("pornography", "pron"),
      ("other", "other"),
    ];

    for (word, dialect) in translations {
      assert_eq!(leetefy_word(word), dialect);
    } 
  }
}