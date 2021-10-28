use crate::ast::root::Root;
use crate::ast::rule::Rule;
use crate::ast::Node;
use crate::input::Input;
use crate::tokenizer::{Token, Tokenizer, TokenType};

pub struct Parser<'a> {
  input: &'a Input,
  current: Box<dyn Node>,
  tokenizer: Tokenizer<'a>,
  spaces: String,
  semicolon: bool,
  custom_property: bool,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a Input) -> Self {
    let root: Root = Root::new(None, None, None, None);
    Self {
      input,
      current: Box::new(root),
      spaces: "".to_string(),
      semicolon: false,
      custom_property: false,
      tokenizer: Tokenizer::new(&input, true),
    }
  }

  pub fn parse(&mut self) {
    use TokenType::*;
    while !self.tokenizer.end_of_file() {
      let token = self.tokenizer.next_token(true);
      match token.kind {
        Space => self.spaces += &token.content,
        Semicolon => self.free_semicolon(&token),
        CloseCurly => self.end(&token),
        Comment => self.comment(&token),
        AtWord => self.atrule(&token),
        OpenCurly => self.empty_rule(&token),
        _ => self.other(&token),
      }
    }
    self.end_file();
  }

  #[inline]
  fn free_semicolon(&mut self, token: &Token) {
    self.spaces += &token.content;
    if let Some(node) = self
      .current
      .nodes_mut()
      .and_then(|nodes| nodes.last_mut())
      .and_then(|prev| prev.as_any_mut().downcast_mut::<&mut Rule>())
    {
      if node.raws.own_semicolon.unwrap_or(false) {
        node.raws.own_semicolon = Some(!self.spaces.is_empty());
        self.spaces = "".to_owned();
      }
    }
  }

  #[inline]
  fn end(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn comment(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn atrule(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn empty_rule(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn other(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn end_file(&self) {
    todo!()
  }
}
