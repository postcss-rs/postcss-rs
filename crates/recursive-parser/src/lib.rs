pub mod parser;
use tokenizer::{Token, TokenType, Tokenizer};

pub(crate) struct Lexer<'a> {
  inner: Tokenizer<'a>,
}

impl<'a> Lexer<'a> {
  pub(crate) fn new(input: &'a str) -> Self {
    Self {
      inner: Tokenizer::new(input, false),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if !self.inner.end_of_file() {
      let token = self.inner.next_token(false);
      Some(token)
    } else {
      None
    }
  }
}
