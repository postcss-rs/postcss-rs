#![allow(unused_variables)]
#![allow(dead_code)]

use crate::input::Input;
use crate::node::{Comment, Node, Position, Root, RootRaws, Rule, RuleRaws};
use crate::regex;
use crate::tokenizer::{Token, TokenType, Tokenizer};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser<'a> {
  pub root: Rc<RefCell<Node<'a>>>,
  current: Rc<RefCell<Node<'a>>>,
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
      root: root.clone(),
      current: root,
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
        _ => self.other(token),
      }
    }
    self.end_file();
  }

  #[inline]
  fn free_semicolon(&mut self, token: &Token) {
    self.spaces += token.1;
    if let Some(rule) = self.current.borrow_mut().as_rule_mut() {
      if rule.raws.own_semicolon.unwrap_or(false) {
        rule.raws.own_semicolon = Some(!self.spaces.is_empty());
        self.spaces = "".to_owned();
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
  fn comment(&mut self, token: &Token) {
    let node = Node::Comment(Comment {
      ..Default::default()
    });
    let node = Rc::new(RefCell::new(node));
    self.init(node.clone(), token.2.expect("expected have start offset"));
    let offset = token
      .3
      .unwrap_or_else(|| token.2.expect("expected have start offset"));
    let (line, column) = self.tokenizer.from_offset(offset);
    node
      .borrow_mut()
      .set_source_end(Some(Position::new(offset, line, column)));

    let text = &token.1[2..token.1.len() - 2];
    if let Some(comment) = node.borrow_mut().as_comment_mut() {
      if is_all_white_space(text) {
        comment.text = Some("".into());
        comment.raws.left = Some(text.to_string());
        comment.raws.right = Some("".to_string());
      } else {
        // text.match(/^(\s*)([^]*\S)(\s*)$/) these two reg is equivalent, but is not valid in rust regexp crate
        let comment_reg = regex!(r#"^(\s*)([\s\S]*\S)(\s*)$"#);
        let capture = comment_reg.captures(text).unwrap();
        comment.text = Some(capture.get(1).unwrap().as_str().to_string());
        comment.raws.left = Some(capture.get(0).unwrap().as_str().to_string());
        comment.raws.right = Some(capture.get(2).unwrap().as_str().to_string());
      }
    };
  }

  #[inline]
  fn atrule(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn empty_rule(&mut self, token: &Token) {
    let node = Node::Rule(Rule {
      selector: "".to_string(),
      raws: RuleRaws {
        between: Some("".to_string()),
        ..Default::default()
      },
      ..Default::default()
    });
    let node = Rc::new(RefCell::new(node));
    self.init(node.clone(), token.2.expect("expected have start offset"));
    self.current = node;
  }

  #[inline]
  fn other(&self, token: Token) {
    todo!()
  }

  #[inline]
  fn end_file(&self) {
    todo!()
  }
  fn get_position(&mut self, offset: usize) -> Position {
    let (line, column) = self.tokenizer.from_offset(offset);
    Position::new(offset, line, column)
  }

  fn init(&mut self, node: Rc<RefCell<Node<'a>>>, offset: usize) {
    let pos = self.get_position(offset);
    let mut cur_node = self.current.borrow_mut();
    cur_node.set_source(self.input.clone(), Some(pos), None);
    cur_node.push_child(node);
    let old_spaces = std::mem::replace(&mut self.spaces, "".to_string());
    cur_node.set_raw_before(old_spaces);
    if !cur_node.is_comment() {
      self.semicolon = false;
    }
  }
}

#[inline]
fn is_all_white_space(s: &str) -> bool {
  s.chars().all(is_whitespace)
}

const fn is_whitespace(ch: char) -> bool {
  matches!(ch as u32, 0x0009..=0x000D
    | 0x0020
    | 0x0085
    | 0x00A0
    | 0x1680
    | 0x2000..=0x200A
    | 0x2028
    | 0x2029
    | 0x202F
    | 0x205F
    | 0x3000)
}
