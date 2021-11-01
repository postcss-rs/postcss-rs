use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::input::Input;
use crate::node::{Node, Raws, RootRaws};
use crate::tokenizer::{Token, TokenType, Tokenizer};

pub struct Parser<'a> {
  input: &'a Input<'a>,
  pub root: RefCell<Node>,
  current: Weak<RefCell<Node>>,
  tokenizer: Tokenizer<'a>,
  spaces: String,
  semicolon: bool,
  custom_property: bool,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a Input) -> Self {
    let root = RefCell::new(Node {
      r#type: "root",
      nodes: todo!(),
      parent: None,
      source: None,
      name: None,
      params: None,
      text: None,
      decl: None,
      rule: None,
      raws: Raws::RootRaws(RootRaws {
        after: None,
        code_before: None,
        code_after: None,
        semicolon: None,
      }),
    });
    let current: Weak<RefCell<Node>> = Weak::new();
    Self {
      input,
      root,
      current,
      spaces: "".to_string(),
      semicolon: false,
      custom_property: false,
      tokenizer: Tokenizer::new(input, true),
    }
  }

  pub fn parse(&mut self) {
    use TokenType::*;
    while !self.tokenizer.end_of_file() {
      let token = self.tokenizer.next_token(true);
      match token.kind {
        Space => self.spaces += &token.1,
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
    self.spaces += &token.1;
    if let Some(node) = self
      .current
      .upgrade()
      .and_then(|node| node.borrow().nodes)
      .and_then(|node| node.get_mut().last_mut())
    {
      match node.raws {
        Raws::RuleRaws(ref mut raws) => {
          if raws.own_semicolon.unwrap_or(false) {
            raws.own_semicolon = Some(!self.spaces.is_empty());
            self.spaces = "".to_owned();
          }
        }
        _ => {}
      }
    }
  }

  //   if let Some(node) = self
  //   .current
  //   .nodes_mut()
  //   .and_then(|nodes| nodes.last_mut())
  //   .and_then(|prev| prev.as_any_mut().downcast_mut::<&mut Rule>())
  // {

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
