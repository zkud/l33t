use std::str::Chars;

#[derive(Debug)]
enum Token {
  Word(Word),
  Punctuation(char),
}

impl Token {
  pub fn leetefy(&self) -> String {
    println!("{:?}", self);
    match self {
      Token::Word(word) => word.leetefy(),
      Token::Punctuation(chr) => String::from(*chr),
    }
  }
}

struct TokenIter<'i> {
  chars: &'i mut Chars<'i>,
  word: Option<Word>,
}

impl<'i> TokenIter<'i> {
  pub fn new(chars: &'i mut Chars<'i>) -> Self {
    TokenIter { chars, word: None }
  }

  fn is_punctuation(chr: char) -> bool {
    chr == '!'
      || chr == ' '
      || chr == '\"'
      || chr == '#'
      || chr == '$'
      || chr == '%'
      || chr == '&'
      || chr == '\''
      || chr == '('
      || chr == ')'
      || chr == '*'
      || chr == '+'
      || chr == ','
      || chr == '-'
      || chr == '.'
      || chr == '/'
      || chr == ':'
      || chr == ';'
      || chr == '<'
      || chr == '='
      || chr == '>'
      || chr == '?'
      || chr == '@'
      || chr == '['
      || chr == '\\'
      || chr == ']'
      || chr == '^'
      || chr == '`'
      || chr == '{'
      || chr == '|'
      || chr == '}'
      || chr == '\n'
      || chr == '\t'
      || chr == '\r'
      || chr == '\0'
  }
}

impl<'i> Iterator for TokenIter<'i> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    for chr in &mut self.chars {
      if Self::is_punctuation(chr) {
        match self.word {
          Some(_) => {
            return self
              .word
              .take()
              .and_then(|word| Some(Token::Word(word)));
          }
          None => return Some(Token::Punctuation(chr)),
        }
      }

      if let Some(word) = &mut self.word {
        word.push(chr);
      } else {
        self.word = Some({
          let mut word = Word::new();
          word.push(chr);
          word
        });
      }
    }
    
    self
      .word
      .take()
      .and_then(|word| Some(Token::Word(word)))
  }
}

#[derive(Debug)]
struct Word {
  content: Vec<char>,
}

impl Word {
  pub fn new() -> Self {
    let mean_length_of_english_word = 5;

    Word {
      content: Vec::with_capacity(mean_length_of_english_word),
    }
  }

  pub fn push(&mut self, input: char) {
    self.content.push(input)
  }

  pub fn leetefy(&self) -> String {
    self
      .content
      .iter()
      .collect()
  }
}

// input -> Parser module (tokenizer) -> generator module -> output
pub fn leetefy_suffixes(line: &str) -> String {
  TokenIter::new(&mut line.chars())
    .map(|token| token.leetefy())
    .collect()
}