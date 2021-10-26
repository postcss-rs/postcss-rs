use crate::ast::root::Root;
use crate::ast::rule::Rule;
use crate::ast::Node;
use crate::input::Input;
use crate::tokenizer::{GetContent, Span, SpanControl, Token, Tokenizer};

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
    while !self.tokenizer.end_of_file() {
      let token = self.tokenizer.next_token(true).unwrap();
      match token {
        Token::Space(it) => self.spaces += it.get_content().as_str(),
        Token::Control(SpanControl {
          symbol: ';',
          content: c,
          ..
        }) => self.free_semicolon(c.as_str()),
        Token::Control(SpanControl {
          symbol: '}',
          content: c,
          ..
        }) => self.end(c.as_str()),
        Token::Control(SpanControl {
          symbol: '{',
          content: c,
          ..
        }) => self.empty_rule(c.as_str()),
        Token::Comment(it) => self.comment(&it),
        Token::AtWord(it) => self.atrule(&it),
        _ => self.other(&token),
      }
    }
    self.end_file();
  }

  #[inline]
  fn free_semicolon(&mut self, token: &str) {
    self.spaces += token;
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
  fn end(&self, token: &str) {
    todo!()
  }

  #[inline]
  fn comment(&self, token: &Span) {
    todo!()
  }

  #[inline]
  fn atrule(&self, token: &Span) {
    todo!()
  }

  #[inline]
  fn empty_rule(&self, token: &str) {
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
