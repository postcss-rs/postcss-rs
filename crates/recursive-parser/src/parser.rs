use std::iter::Peekable;
use tokenizer::{Token, TokenType, Tokenizer};

use crate::Lexer;
pub struct Root<'a> {
  children: Vec<RuleOrAtRuleOrDecl<'a>>,
  start: usize,
  end: usize,
}

enum RuleOrAtRuleOrDecl<'a> {
  Rule(Rule<'a>),
  AtRule(AtRule<'a>),
  Declaration(Declaration<'a>),
}

// enum AtRuleOrDeclaration<'a> {
//   Declaration(Declaration<'a>),
//   AtRule(AtRule<'a>),
// }
pub struct Rule<'a> {
  children: Vec<RuleOrAtRuleOrDecl<'a>>,
  start: usize,
  end: usize,
}

pub struct Declaration<'a> {
  prop: Prop<'a>,
  value: Value<'a>,
}

pub struct Prop<'a> {
  content: &'a str,
  start: usize,
  end: usize,
}

pub struct Value<'a> {
  content: &'a str,
  start: usize,
  end: usize,
}

pub struct AtRule<'a> {
  selector: Selector<'a>,
  start: usize,
  end: usize,
}
struct Selector<'a> {
  content: &'a str,
  start: usize,
  end: usize,
}

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  pos: usize,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      lexer: Lexer::new(input).peekable(),
      pos: 0,
    }
  }

  pub fn parse(mut self) -> Root<'a> {
    // self.parse_element();
    let mut children = vec![];
    while let Some(syntax) = self.peek() {
      match syntax {
        TokenType::Space => {
          self.bump();
        }
        TokenType::AtWord => {
          // println!("parse at rule top level");
          self.parse_at_rule();
        }
        TokenType::Comment => {
          self.parse_comment();
        }
        _ => {
          self.parse_rule();
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
    let rule: Rule;
    let start = self.pos;
    if let Some(kind) = self.peek() {
      match kind {
        TokenType::OpenCurly => {
          let children = self.parse_curly_block(false);
          Rule {
            children,
            start,
            end: 0,
          }
        }
        _ => {
          self.parse_component();
          loop {
            match self.peek() {
              Some(kind) => match kind {
                TokenType::OpenCurly => {
                  // let children = 
                  return Rule {
                    children: self.parse_curly_block(false),
                    start,
                    end: self.pos,
                  }
                }
                TokenType::Space => {
                  self.bump();
                }
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
    } else {
      unimplemented!("should parse a Rule")
    }
  }
  // https://drafts.csswg.org/css-syntax/#component-value-diagram
  #[inline]
  fn parse_component(&mut self) {
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

  fn parse_curly_block(&mut self, rule: bool) -> Vec<RuleOrAtRuleOrDecl<'a>> {
    use TokenType::*;
    // println!("parse curlyblock");
    let mut ret: Vec<RuleOrAtRuleOrDecl> = vec![];
    self.bump(); // bump {
    self.skip_whitespace();
    loop {
      match self.peek() {
        Some(kind) => match kind {
          Semicolon => {
            self.bump();
          }
          AtWord => ret.push(RuleOrAtRuleOrDecl::AtRule(self.parse_at_rule())),
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
      content,
      start,
      end,
    };
    self.skip_whitespace();
    assert!(
      matches!(self.peek(), Some(TokenType::Colon)),
      "expected : found {:?}",
      self.peek()
    );
    self.bump();
    self.skip_whitespace();
    let mut has_finish = false;
    let mut value: Value;
    while let Some(kind) = self.peek() {
      match kind {
        CloseCurly | Semicolon => {
          has_finish = true;
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
    if !has_finish {}
    Declaration {
      prop,
      value: todo!(),
    }
  }

  pub fn parse_at_rule(&mut self) -> AtRule<'a> {
    use TokenType::*;
    self.bump(); // bump atWord
    self.skip_whitespace();
    while let Some(kind) = self.peek() {
      match kind {
        OpenCurly => {
          //   self.finish_node(); finish params
          self.parse_curly_block(true);
          break;
        }
        Semicolon => {
          //   self.finish_node();
          self.bump();
          break;
        }
        _ => {
          self.parse_component();
        }
      }
    }
    AtRule {
      selector: todo!(),
      start: todo!(),
      end: todo!(),
    }
  }

  #[inline]
  pub fn skip_whitespace(&mut self) {
    if let Some(TokenType::Space) = self.peek() {
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
