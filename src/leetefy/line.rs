use super::char::leetefy_char;
use super::word::leetefy_word;
use peeking_take_while::PeekableExt;
use std::iter::Peekable;

struct Tokens<'i> {
  chars: Peekable<&'i mut dyn Iterator<Item = char>>,
}

impl<'i> Iterator for Tokens<'i> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    let word: String = self
      .chars
      .peeking_take_while(|c| !Self::is_punctuation(*c))
      .collect();

    if !word.is_empty() {
      Some(Token::Word(word))
    } else {
      self.chars.next().and_then(|c| Some(Token::Punctuation(c)))
    }
  }
}

impl<'i> Tokens<'i> {
  pub fn new(chars: &'i mut dyn Iterator<Item = char>) -> Self {
    Tokens {
      chars: chars.peekable(),
    }
  }

  fn is_punctuation(chr: char) -> bool {
    const PUNCTUATION_CHARS: &[char] = &[
      '!', ' ', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';',
      '<', '=', '>', '?', '@', '[', '\\', ']', '^', '`', '{', '|', '}', '\n', '\t', '\r', '\0',
    ];

    PUNCTUATION_CHARS.contains(&chr)
  }
}

enum Token {
  Word(String),
  Punctuation(char),
}

impl Token {
  pub fn leetefy(&self) -> String {
    match self {
      Token::Word(word) => leetefy_word(&word)
        .chars()
        .map(|chr| leetefy_char(&chr))
        .collect(),
      Token::Punctuation(chr) => String::from(*chr),
    }
  }
}

pub fn leetefy_line(line: &str) -> String {
  Tokens::new(&mut line.chars())
    .map(|token| token.leetefy())
    .collect()
}
