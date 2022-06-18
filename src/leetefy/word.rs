use super::random::maybe_translate;

pub fn leetefy_word(word: &str) -> String {
  let possibility = 0.2;
  match word {
    and_word if is_and_word(&and_word) => {
      let translation = translate_and(&and_word);
      maybe_translate(and_word, vec![translation.as_str()], possibility)
    }
    "own" | "gain" => maybe_translate(word, vec!["pwn"], possibility),
    "owned" | "gained" => maybe_translate(word, vec!["pwn'd"], possibility),
    "owning" | "gaining" => maybe_translate(word, vec!["pwng"], possibility),
    "soft" | "software" => maybe_translate(word, vec!["warez"], possibility),
    "hacker" => maybe_translate(word, vec!["hackxor"], possibility),
    "newbie" => maybe_translate(word, vec!["noob"], possibility),
    "porn" | "pornography" => maybe_translate(word, vec!["pron"], possibility),
    _ => String::from(word),
  }
}

#[inline]
fn is_and_word(word: &str) -> bool {
  word.contains("and") || word.contains("ant")
}

#[inline]
fn translate_and(word: &str) -> String {
  word.replace("ant", "&").replace("and", "&")
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
      let translation = leetefy_word(word);
      assert!(translation == dialect || translation == word);
    }
  }
}
