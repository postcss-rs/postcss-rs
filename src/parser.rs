use crate::ast::root::Root;
use crate::input::Input;
use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub struct Parser<'a> {
  input: &'a Input,
  current: Root,
  tokenizer: Tokenizer<'a>,
  spaces: String,
  semicolon: bool,
  custom_property: bool,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a Input) -> Self {
    let root = Root::new(None, None, None, None);
    Self {
      input,
      current: root,
      spaces: "".to_string(),
      semicolon: false,
      custom_property: false,
      tokenizer: Tokenizer::new(&input, true),
    }
  }

  pub fn parse(&mut self) {
    while !self.tokenizer.end_of_file() {
      let token = self.tokenizer.next_token(true);
      match token.0 {
        "space" => self.spaces += &token.1,
        ";" => self.free_semicolon(&token),
        "}" => self.end(&token),
        "comment" => self.comment(&token),
        "at-word" => self.atrule(&token),
        "{" => self.empty_rule(&token),
        _ => self.other(&token),
      }
    }
    self.end_file();
  }

  fn free_semicolon(&mut self, token: &Token) {
    self.spaces += &token.1;
    todo!()
  }

  fn end(&self, token: &Token) {
    todo!()
  }

  fn comment(&self, token: &Token) {
    todo!()
  }

  fn atrule(&self, token: &Token) {
    todo!()
  }

  fn empty_rule(&self, token: &Token) {
    todo!()
  }

  fn other(&self, token: &Token) {
    todo!()
  }

  fn end_file(&self) {
    todo!()
  }
}
