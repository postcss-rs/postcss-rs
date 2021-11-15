// use rowan_parser::parser::Parser;

use std::time::Instant;

use rowan::WalkEvent;
use rowan_parser::{
  parser::Parser,
  syntax::{Lang, SyntaxNode},
};

// fn parse_and_serialize(input: &str) -> () {
//   let mut input = ParserInput::new(input);
//   let mut parser = CssParser::new(&mut input);
//   // let mut serialization = String::new();
//   loop {
//     let loc = parser.position();
//     let token = parser.next_including_whitespace_and_comments();
//     let len = token.as_ref().unwrap();
//     println!("loc: {:?}, {:?}", loc, token);
//     match token {
//       Ok(token) => match token {
//         Token::Ident(_) => (),
//         Token::AtKeyword(_) => (),
//         Token::Hash(_) => (),
//         Token::IDHash(_) => (),
//         Token::QuotedString(_) => (),
//         Token::UnquotedUrl(_) => (),
//         Token::Delim(_) => (),
//         Token::Number {
//           has_sign,
//           value,
//           int_value,
//         } => (),
//         Token::Percentage {
//           has_sign,
//           unit_value,
//           int_value,
//         } => (),
//         Token::Dimension {
//           has_sign,
//           value,
//           int_value,
//           unit,
//         } => (),
//         Token::WhiteSpace(_) => (),
//         Token::Comment(_) => (),
//         Token::Colon => (),
//         Token::Semicolon => (),
//         Token::Comma => (),
//         Token::IncludeMatch => (),
//         Token::DashMatch => (),
//         Token::PrefixMatch => (),
//         Token::SuffixMatch => (),
//         Token::SubstringMatch => (),
//         Token::CDO => (),
//         Token::CDC => (),
//         Token::Function(a) => (),
//         Token::ParenthesisBlock => (),
//         Token::SquareBracketBlock => (),
//         Token::CurlyBracketBlock => {}
//         Token::BadUrl(_) => (),
//         Token::BadString(_) => (),
//         Token::CloseParenthesis => (),
//         Token::CloseSquareBracket => (),
//         Token::CloseCurlyBracket => (),
//       },
//       Err(err) => break,
//     };
//   }
// }
fn main() {
  let css = include_str!("../../../assets/bootstrap.css");
  let code = r#"/**
 * Paste or drop some CSS here and explore
 * the syntax tree created by chosen parser.
 * Enjoy!
 */

@media screen and (min-width: 480px) {
    body, resulkt, .result {
        background-color: lightgreen;
    }
}

#main {
    border: 1px solid black;
}

ul li {
	padding: 5px;
}

"#;
  let instant = Instant::now();
  let parser = Parser::new(css);
  let node = parser.parse().green_node;
  // let ins = Instant::now();
  let _lang = SyntaxNode::new_root(node);
  // println!("{:?}", ins.elapsed());
  // // let _res = format!("{}", lang);
  // // lang.to_string();

  println!("{:?}", instant.elapsed());
  // println!("{:#?}", _lang);
  // let start = Instant::now();
  // let _res = format!("{}", lang);
  // assert_eq!(_res, css);
  // println!("{:?}", start.elapsed());

  // // println!("{:#?}", lang);
  // let start = Instant::now();
  // let mut string = String::with_capacity(0);
  // let mut id = 0;
  // stringify(&lang, &mut id);
  // // assert_eq!(_res, css);
  // println!("{:?}", id);
  // println!("1 {:?}", start.elapsed());
  // // println!("{}", string);
  let start = Instant::now();
  let mut string = String::with_capacity(0);
  // let mut id = 0;
  reverse_plugin(_lang, &mut string, css);
  // // assert_eq!(_res, css);
  // println!("{:?}", id);
  println!("reverse plugin{:?}", start.elapsed());
}

fn reverse_plugin(root: SyntaxNode, string: &mut String, source: &str) {
  root.preorder().for_each(|e| match e {
    WalkEvent::Enter(n) => match n.kind() {
      rowan_parser::syntax::SyntaxKind::Document
      | rowan_parser::syntax::SyntaxKind::Declaration
      | rowan_parser::syntax::SyntaxKind::AtRule
      | rowan_parser::syntax::SyntaxKind::Rule
      | rowan_parser::syntax::SyntaxKind::Selector
      | rowan_parser::syntax::SyntaxKind::Params
      | rowan_parser::syntax::SyntaxKind::Value
      | rowan_parser::syntax::SyntaxKind::Comment => {
        string.push_str(&source[n.text_range()]);
      }
      rowan_parser::syntax::SyntaxKind::Prop => {
        string.push_str(&source[n.text_range()].chars().rev().collect::<String>());
      }
      _ => {}
    },
    WalkEvent::Leave(_) => {}
  });
}
