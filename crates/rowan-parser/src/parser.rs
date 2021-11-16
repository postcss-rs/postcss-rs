use std::iter::Peekable;

use crate::syntax::Lang;
use crate::syntax::{Lexer, SyntaxKind};
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      lexer: Lexer::new(input).peekable(),
      builder: GreenNodeBuilder::new(),
    }
  }

  pub fn parse(mut self) -> Parse {
    self.builder.start_node(SyntaxKind::Root.into());
    // self.parse_element();
    while let Some(syntax) = self.peek() {
      match syntax {
        SyntaxKind::Space => self.bump(),
        SyntaxKind::AtWord => {
          // println!("parse at rule top level");
          self.parse_at_rule()
        }
        SyntaxKind::Comment => self.parse_comment(),
        _ => {
          self.parse_rule();
        }
      }
    }
    self.builder.finish_node();
    Parse {
      green_node: self.builder.finish(),
    }
  }

  #[inline]
  pub fn parse_comment(&mut self) {
    self.start_node(SyntaxKind::Comment);
    self.bump();
    self.finish_node();
  }

  #[inline]
  pub fn parse_rule(&mut self) {
    self.start_node(SyntaxKind::Rule);
    if let Some(kind) = self.peek() {
      match kind {
        SyntaxKind::OpenCurly => {
          self.parse_curly_block(false);
        }
        _ => {
          self.start_node(SyntaxKind::Selector);
          self.parse_component();
          loop {
            match self.peek() {
              Some(kind) => match kind {
                SyntaxKind::OpenCurly => {
                  self.finish_node();
                  self.parse_curly_block(false);
                  break;
                }
                SyntaxKind::Space => self.bump(),
                _ => {
                  self.parse_component();
                }
              },
              None => {
                panic!(r#"expected {} found none"#, "{");
              }
            }
          }
        }
      }
    }
    self.finish_node();
  }
  // https://drafts.csswg.org/css-syntax/#component-value-diagram
  #[inline]
  fn parse_component(&mut self) {
    // self.start_node(SyntaxKind::Component);
    if let Some(kind) = self.peek() {
      match kind {
        SyntaxKind::OpenParentheses => {
          // println!("parse open parentheses");
          self.parse_parentheses_block();
        }
        SyntaxKind::OpenSquare => {
          self.parse_square_block();
        }
        SyntaxKind::OpenCurly => {
          self.parse_curly_block(false);
        }
        _ => {
          // println!("need to bump {:?} from parse component", self.peek());
          self.bump();
        }
      }
    }
    // self.finish_node();
  }

  fn parse_parentheses_block(&mut self) {
    self.bump(); // bump (
    loop {
      match self.peek() {
        Some(kind) => match kind {
          SyntaxKind::CloseParentheses => {
            self.bump();
            break;
          }
          _ => {
            self.parse_component();
          }
        },
        None => {
          // TODO: error handle
          panic!("expected ) found none");
        }
      }
    }
  }

  fn parse_square_block(&mut self) {
    self.bump(); // bump [
    loop {
      match self.peek() {
        Some(kind) => match kind {
          SyntaxKind::CloseSquare => {
            self.bump();
            break;
          }
          _ => {
            self.parse_component();
          }
        },
        None => {
          // TODO: error handle
          panic!("expected ] found none");
        }
      }
    }
  }

  fn parse_curly_block(&mut self, rule: bool) {
    use SyntaxKind::*;
    // println!("parse curlyblock");
    self.bump(); // bump {
    self.skip_whitespace();
    loop {
      match self.peek() {
        Some(kind) => match kind {
          Semicolon => self.bump(),
          AtWord => self.parse_at_rule(),
          Space => {
            self.bump();
          }
          CloseCurly => {
            self.bump();
            // println!("finish close curly");
            break;
          }
          _ => {
            if rule {
              // println!("parse rule -->");
              self.parse_rule();
            } else {
              // println!("parse declaration");
              self.parse_declaration();
            }
          }
        },
        None => {
          //TODO: error handle
          panic!("expected close curly");
        }
      }
    }
  }

  fn parse_declaration(&mut self) {
    use SyntaxKind::*;
    self.start_node(Declaration);
    assert!(
      matches!(self.peek(), Some(Word)),
      "expected word found {:?}",
      self.peek(),
    );
    self.start_node(SyntaxKind::Prop);
    self.bump();
    self.finish_node();
    self.skip_whitespace();
    assert!(
      matches!(self.peek(), Some(SyntaxKind::Colon)),
      "expected : found {:?}",
      self.peek()
    );
    self.bump();
    self.skip_whitespace();
    self.start_node(SyntaxKind::Value);
    let mut has_finish = false;
    while let Some(kind) = self.peek() {
      match kind {
        CloseCurly | Semicolon => {
          has_finish = true;
          self.finish_node();
          break;
        }
        Space => self.bump(),
        _ => {
          // println!("parse the component");
          self.parse_component();
        }
      }
    }
    if !has_finish {
      self.finish_node();
    }
    self.finish_node();
  }

  pub fn parse_at_rule(&mut self) {
    use SyntaxKind::*;
    self.start_node(SyntaxKind::AtRule);
    self.bump(); // bump atWord
    self.skip_whitespace();
    self.start_node(SyntaxKind::Params);
    while let Some(kind) = self.peek() {
      match kind {
        OpenCurly => {
          self.finish_node();
          self.parse_curly_block(true);
          break;
        }
        Semicolon => {
          self.finish_node();
          self.bump();
          break;
        }
        _ => {
          self.parse_component();
        }
      }
    }
    self.finish_node();
  }

  #[inline]
  pub fn skip_whitespace(&mut self) {
    if let Some(SyntaxKind::Space) = self.peek() {
      self.bump();
    }
  }
  pub fn peek(&mut self) -> Option<SyntaxKind> {
    self.lexer.peek().map(|(kind, _)| *kind)
  }

  pub fn bump(&mut self) {
    let (kind, text) = self.lexer.next().unwrap();
    // println!("{:?}, {:?}", kind, text);
    self.builder.token(Lang::kind_to_raw(kind), text.into());
  }

  fn start_node(&mut self, kind: SyntaxKind) {
    self.builder.start_node(Lang::kind_to_raw(kind));
  }

  fn finish_node(&mut self) {
    self.builder.finish_node();
  }
}

pub struct Parse {
  pub green_node: GreenNode,
}
