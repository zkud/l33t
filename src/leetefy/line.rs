use super::char;
use super::suffix;
use std::str::Chars;

struct Tokens<'i> {
  chars: &'i mut Chars<'i>,
  word: Option<Word>,
  punctuation: Option<char>,
}

impl<'i> Tokens<'i> {
  pub fn new(chars: &'i mut Chars<'i>) -> Self {
    Tokens {
      chars,
      word: None,
      punctuation: None,
    }
  }

  fn is_punctuation(chr: char) -> bool {
    const PUNCTUATION_CHARS: &[char] = &[
      '!', ' ', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';',
      '<', '=', '>', '?', '@', '[', '\\', ']', '^', '`', '{', '|', '}', '\n', '\t', '\r', '\0',
    ];

    PUNCTUATION_CHARS.contains(&chr)
  }

  fn push_char(word: &mut Option<Word>, chr: char) {
    if let Some(word) = word {
      word.push(chr);
    } else {
      *word = Some({
        let mut word = Word::new();
        word.push(chr);
        word
      });
    }
  }
}

impl<'i> Iterator for Tokens<'i> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(_) = self.punctuation {
      return self
        .punctuation
        .take()
        .and_then(|chr| Some(Token::Punctuation(chr)));
    }

    for chr in &mut self.chars {
      if Self::is_punctuation(chr) {
        match self.word {
          Some(_) => {
            self.punctuation = Some(chr);
            return self.word.take().and_then(|word| Some(Token::Word(word)));
          }
          None => return Some(Token::Punctuation(chr)),
        }
      }

      Self::push_char(&mut self.word, chr);
    }

    self.word.take().and_then(|word| Some(Token::Word(word)))
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
  pub fn new() -> Self {
    const MEAN_LENGTH_OF_ENGLISH_WORD: usize = 5;
    Word {
      content: Vec::with_capacity(MEAN_LENGTH_OF_ENGLISH_WORD),
    }
  }

  pub fn push(&mut self, input: char) {
    self.content.push(input)
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
