#![allow(unused_variables)]
#![allow(dead_code)]

use crate::input::Input;
use crate::node::{Node, Position, Root, RootRaws};
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
  input: Rc<RefCell<Input<'a>>>,
}

impl<'a> Parser<'a> {
  pub fn new(input: Input<'a>) -> Self {
    let root = Rc::new(RefCell::new(Node::Root(Root {
      nodes: Some(vec![]),
      parent: None,
      source: None,
      raws: RootRaws::default(),
    })));
    Self {
      root,
      current: None,
      spaces: "".to_string(),
      semicolon: false,
      custom_property: false,
      tokenizer: Tokenizer::new(input.css, true),
      input: Rc::new(RefCell::new(input)),
    }
  }

  pub fn parse(&mut self) {
    use TokenType::*;
    while !self.tokenizer.end_of_file() {
      let token = self.tokenizer.next_token(true);
      match token.0 {
        Space => self.spaces += token.1,
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
    self.spaces += token.1;
    if let Some(ref mut node) = self.current {
      if let Node::Rule(ref mut rule) = node {
        if rule.raws.own_semicolon.unwrap_or(false) {
          rule.raws.own_semicolon = Some(!self.spaces.is_empty());
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
  fn get_position(&mut self, offset: usize) -> Position {
    let (line, column) = self.tokenizer.from_offset(offset);
    Position::new(offset, column, line)
  }

  fn init(&mut self, node: Node<'a>, offset: usize) {
    use crate::node::Node::*;
    let pos = self.get_position(offset);
    if let Some(ref mut cur_node) = self.current {
      cur_node.set_source(self.input.clone(), Some(pos), None);
      cur_node.push_child(node);
      let old_spaces = std::mem::replace(&mut self.spaces, "".to_string());
      cur_node.set_raw_before(old_spaces);
      if !matches!(cur_node, Comment(_)) {
        self.semicolon = false;
      }
    }
  }
}
