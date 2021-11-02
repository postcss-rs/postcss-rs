#![allow(unused_variables)]
#![allow(dead_code)]

use crate::input::Input;
use crate::node::{Node, Raws, RootRaws};
use crate::tokenizer::{Token, TokenType, Tokenizer};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser<'a> {
  pub root: Rc<RefCell<Node<'a>>>,
  current: Option<Node<'a>>,
  tokenizer: Tokenizer<'a>,
  spaces: String,
  semicolon: bool,
  custom_property: bool,
}

impl<'a> Parser<'a> {
  pub fn new(input: Input<'a>) -> Self {
    let root = Rc::new(RefCell::new(Node {
      r#type: "root",
      nodes: None,
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
    }));
    Self {
      root,
      current: None,
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
      match token.0 {
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
    if let Some(ref mut node) = self.current {
      if let Raws::RuleRaws(ref mut raws) = node.raws {
        if raws.own_semicolon.unwrap_or(false) {
          raws.own_semicolon = Some(!self.spaces.is_empty());
          self.spaces = "".to_owned();
        }
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
