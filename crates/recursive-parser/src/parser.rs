use std::{borrow::Cow, iter::Peekable, ops::Add};
use tokenizer::{Token, TokenType, Tokenizer};

use crate::syntax::Lexer;
pub struct Root<'a> {
  pub children: Vec<RuleOrAtRuleOrDecl<'a>>,
  pub(crate) start: usize,
  pub(crate) end: usize,
}

pub enum RuleOrAtRuleOrDecl<'a> {
  Rule(Rule<'a>),
  AtRule(AtRule<'a>),
  Declaration(Declaration<'a>),
}

// enum AtRuleOrDeclaration<'a> {
//   Declaration(Declaration<'a>),
//   AtRule(AtRule<'a>),
// }
pub struct Rule<'a> {
  pub children: Vec<RuleOrAtRuleOrDecl<'a>>,
  pub start: usize,
  pub end: usize,
  pub selector: Selector<'a>,
}

pub struct Declaration<'a> {
  pub(crate) prop: Prop<'a>,
  pub(crate) value: Value<'a>,
  pub(crate) start: usize,
  pub(crate) end: usize,
}

pub struct Prop<'a> {
  pub(crate) content: Cow<'a, str>,
  pub start: usize,
  pub end: usize,
}

pub struct Value<'a> {
  pub content: Cow<'a, str>,
  pub start: usize,
  pub end: usize,
}

pub struct AtRule<'a> {
  pub params: Cow<'a, str>,
  pub name: Cow<'a, str>,
  pub(crate) start: usize,
  pub(crate) end: usize,
  pub(crate) children: Vec<RuleOrAtRuleOrDecl<'a>>,
}
pub struct Selector<'a> {
  pub content: Cow<'a, str>,
  pub start: usize,
  pub end: usize,
}

impl<'a> Selector<'a> {
  fn new(content: Cow<'a, str>, start: usize, end: usize) -> Self {
    Self {
      content,
      start,
      end,
    }
  }
}

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  source: &'a str,
  pos: usize,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      lexer: Lexer::new(input).peekable(),
      source: input,
      pos: 0,
    }
  }

  pub fn parse(mut self) -> Root<'a> {
    // self.parse_element();
    let mut children: Vec<RuleOrAtRuleOrDecl> = vec![];
    while let Some(syntax) = self.peek() {
      match syntax {
        TokenType::Space => {
          self.bump();
        }
        TokenType::AtWord => {
          children.push(RuleOrAtRuleOrDecl::AtRule(self.parse_at_rule()));
        }
        TokenType::Comment => {
          self.parse_comment();
        }
        _ => {
          children.push(RuleOrAtRuleOrDecl::Rule(self.parse_rule()));
        }
      };
    }
    Root {
      children,
      start: 0,
      end: self.pos,
    }
  }

  #[inline]
  pub fn parse_comment(&mut self) {
    self.bump();
  }

  #[inline]
  pub fn parse_rule(&mut self) -> Rule<'a> {
    let start = self.pos;
    if let Some(kind) = self.peek() {
      match kind {
        TokenType::OpenCurly => {
          let children = self.parse_curly_block(false);
          Rule {
            selector: Selector::new(Cow::Borrowed(""), start, start),
            children,
            start,
            end: self.pos,
          }
        }
        _ => {
          self.parse_component();
          let mut selector_end = self.pos;
          loop {
            match self.peek() {
              Some(kind) => match kind {
                TokenType::OpenCurly => {
                  return Rule {
                    selector: Selector::new(
                      Cow::Borrowed(&self.source[start..selector_end]),
                      start,
                      selector_end,
                    ),
                    children: self.parse_curly_block(false),
                    start,
                    end: self.pos,
                  };
                }
                TokenType::Space | TokenType::Comment => {
                  self.bump();
                }
                _ => {
                  self.parse_component();
                  selector_end = self.pos;
                }
              },
              None => {
                panic!(r#"expected {} found none"#, "{");
              }
            }
          }
        }
      }
    } else {
      unimplemented!("should parse a Rule")
    }
  }
  // https://drafts.csswg.org/css-syntax/#component-value-diagram
  #[inline]
  /// return bump token is trivial
  fn parse_component(&mut self) -> bool {
    // self.start_node(TokenType::Component);
    if let Some(kind) = self.peek() {
      match kind {
        TokenType::OpenParentheses => {
          // println!("parse open parentheses");
          self.parse_parentheses_block();
        }
        TokenType::OpenSquare => {
          self.parse_square_block();
        }
        TokenType::OpenCurly => {
          self.parse_curly_block_in_component();
        }
        _ => {
          // println!("need to bump {:?} from parse component", self.peek());
          return matches!(self.bump().0, TokenType::Space | TokenType::Comment);
        }
      }
    } else {
      eprintln!("expected token found none");
    }
    false
    // self.finish_node();
  }

  fn parse_parentheses_block(&mut self) {
    self.bump(); // bump (
    loop {
      match self.peek() {
        Some(kind) => match kind {
          TokenType::CloseParentheses => {
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
          TokenType::CloseSquare => {
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

  fn parse_curly_block_in_component(&mut self) {
    self.bump(); // bump {
    loop {
      match self.peek() {
        Some(kind) => match kind {
          TokenType::CloseCurly => {
            self.bump();
            break;
          }
          _ => {
            self.parse_component();
          }
        },
        None => {
          // TODO: error handle
          panic!("expected {} found none", "}");
        }
      }
    }
  }

  fn parse_curly_block(&mut self, rule: bool) -> Vec<RuleOrAtRuleOrDecl<'a>> {
    use TokenType::*;
    // println!("parse curlyblock");
    let mut ret: Vec<RuleOrAtRuleOrDecl> = vec![];
    self.bump(); // bump {
    self.skip_whitespace_comment();
    loop {
      match self.peek() {
        Some(kind) => match kind {
          Semicolon => {
            self.bump();
          }
          AtWord => ret.push(RuleOrAtRuleOrDecl::AtRule(self.parse_at_rule())),
          Space | Comment => {
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
              ret.push(RuleOrAtRuleOrDecl::Rule(self.parse_rule()));
            } else {
              // println!("parse declaration");
              ret.push(RuleOrAtRuleOrDecl::Declaration(self.parse_declaration()));
            }
          }
        },
        None => {
          //TODO: error handle
          panic!("expected close curly");
        }
      }
    }
    ret
  }

  fn parse_declaration(&mut self) -> Declaration<'a> {
    use TokenType::*;
    assert!(
      matches!(self.peek(), Some(Word)),
      "expected word found {:?}",
      self.peek(),
    );
    let Token(_, content, start, end) = self.bump();
    let prop = Prop {
      content: Cow::Borrowed(content),
      start,
      end,
    };
    self.skip_whitespace_comment();
    assert!(
      matches!(self.peek(), Some(TokenType::Colon)),
      "expected : found {:?}",
      self.peek()
    );
    self.bump();
    self.skip_whitespace_comment();
    let mut has_finish = false;
    let mut value = Value {
      content: Cow::Borrowed(""),
      start: self.pos,
      end: 0,
    };
    while let Some(kind) = self.peek() {
      match kind {
        CloseCurly | Semicolon => {
          has_finish = true;
          value.end = self.pos;
          value.content = Cow::Borrowed(&self.source[value.start..value.end]);
          break;
        }
        Space => {
          self.bump();
        }
        _ => {
          // println!("parse the component");
          self.parse_component();
        }
      }
    }
    if !has_finish {
      value.end = self.pos;
      value.content = Cow::Borrowed(&self.source[value.start..value.end]);
    }
    let end = if matches!(self.peek(), Some(Semicolon)) {
      self.lexer.peek().unwrap().3
    } else {
      value.end
    };
    Declaration {
      start: prop.start,
      end,
      prop,
      value,
    }
  }

  pub fn parse_at_rule(&mut self) -> AtRule<'a> {
    use TokenType::*;
    let start = self.pos;
    let Token(_, name, _, _) = self.bump(); // bump atWord
    self.skip_whitespace_comment();
    let mut children = vec![];
    let params_start = self.pos;
    let mut params_end = self.pos;
    while let Some(kind) = self.peek() {
      match kind {
        OpenCurly => {
          //   self.finish_node(); finish params
          children = self.parse_curly_block(true);
          break;
        }
        Semicolon => {
          //   self.finish_node();
          self.bump();
          break;
        }
        _ => {
          if !self.parse_component() {
            params_end = self.pos;
          }
        }
      }
    }
    AtRule {
      params: Cow::Borrowed(&self.source[params_start..params_end]),
      name: Cow::Borrowed(name),
      start,
      end: self.pos,
      children,
    }
  }

  #[inline]
  pub fn skip_whitespace_comment(&mut self) {
    while matches!(self.peek(), Some(TokenType::Space | TokenType::Comment)) {
      self.bump();
    }
  }
  pub fn peek(&mut self) -> Option<TokenType> {
    self.lexer.peek().map(|token| token.0)
  }

  pub fn bump(&mut self) -> Token<'a> {
    let token = self.lexer.next().unwrap();
    self.pos = token.3;
    token
    // println!("{:?}, {:?}", kind, text);
  }
}
