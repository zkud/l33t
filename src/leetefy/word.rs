pub fn leetefy_word(word: &String) -> String {
  match word.as_str() {
    "soft" | "software" => String::from("warez"),
    "hacker" => String::from("hackxor"),
    "newbie" => String::from("noob"),
    "porn" | "pornography" => String::from("pron"),
    _ => word.clone(),
  }
}
