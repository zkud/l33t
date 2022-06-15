use super::char;
use super::suffix;
use std::iter::Peekable;
use peeking_take_while::PeekableExt;

struct Tokens<'i> {
  chars: Peekable<&'i mut dyn Iterator<Item = char>> 
}

impl<'i> Iterator for Tokens<'i> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    let word: Vec<char> = 
      self.chars
      .peeking_take_while(|c| !Self::is_punctuation(*c))
      .collect();
    
    if !word.is_empty() {
      return Some(Token::Word(Word::new(word)));
    } else {
      match self.chars.next() {
        Some(c) if Self::is_punctuation(c) => Some(Token::Punctuation(c)),
        _ => None
      }
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
  Word(Word),
  Punctuation(char),
}

impl Token {
  pub fn leetefy(&self) -> String {
    match self {
      Token::Word(word) => word.leetefy(),
      Token::Punctuation(chr) => String::from(*chr),
    }
  }
}

struct Word {
  content: Vec<char>,
}

impl Word {
  pub fn new(content: Vec<char>) -> Self {
    Word {
      content,
    }
  }

  pub fn leetefy(&self) -> String {
    suffix::leetefy_suffix(&self.content)
      .iter()
      .map(|chr| char::leetefy_char(chr))
      .collect()
  }
}

pub fn leetefy_line(line: &str) -> String {
  Tokens::new(&mut line.chars())
    .map(|token| token.leetefy())
    .collect()
}
