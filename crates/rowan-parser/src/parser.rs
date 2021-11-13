use std::iter::Peekable;

// use crate::syntax::{Lexer, SyntaxKind};
// use crate::syntax::Lang;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub struct Parser {
  // lexer: Peekable<Lexer<'a>>,
// builder: GreenNodeBuilder<'static>,
}

impl Parser {
  pub fn new(input: &str) -> Self {
    Self {
            // lexer: Parser::new(input).peekable(),
            // builder: GreenNodeBuilder::new(),
        }
  }

  // pub fn parse(mut self) -> Parse {
  //     self.builder.start_node(SyntaxKind::Root.into());
  //     self.parse_element();
  //     self.builder.finish_node();
  //     Parse {
  //         green_node: self.builder.finish(),
  //     }
  // }

  // pub fn parse_element(&mut self) {
  //     self.skip_whitespace();

  //     self.skip_whitespace();
  // }
  // pub fn parse_member(&mut self) {
  //     self.skip_whitespace();
  //     match self.peek() {
  //         Some(SyntaxKind::String) => {
  //             self.bump();
  //         }
  //         None => todo!(),
  //         _ => {
  //             let res = self.lexer.next().unwrap();
  //             panic!("{:?}", res);
  //         }
  //     }
  //     self.skip_whitespace();
  //     assert!(matches!(self.peek(), Some(SyntaxKind::Colon)));
  //     self.bump();
  //     self.parse_element();
  // }

  // pub fn skip_whitespace(&mut self) {
  //     while let Some(SyntaxKind::Space) = self.peek() {
  //         self.bump();
  //     }
  // }
  // pub fn peek(&mut self) -> Option<SyntaxKind> {
  //     self.lexer.peek().map(|(kind, _)| *kind)
  // }

  // fn bump(&mut self) {
  //     let (kind, text) = self.lexer.next().unwrap();

  //     self.builder.token(Lang::kind_to_raw(kind), text.into());
  // }
  // fn start_node(&mut self, kind: SyntaxKind) {
  //     self.builder.start_node(Lang::kind_to_raw(kind));
  // }

  // fn finish_node(&mut self) {
  //     self.builder.finish_node();
  // }
}

pub struct Parse {
  pub green_node: GreenNode,
}
