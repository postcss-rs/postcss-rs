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
  let code = r#".test that [] {
    width : 10px
  }
"#;
  let instant = Instant::now();
  let parser = Parser::new(css);
  // println!("{:?}", parser.peek());
  let node = parser.parse().green_node;
  let ins = Instant::now();
  let lang = SyntaxNode::new_root(node);
  println!("{:?}", ins.elapsed());
  // let _res = format!("{}", lang);
  // lang.to_string();

  println!("{:?}", instant.elapsed());

  let start = Instant::now();
  let _res = format!("{}", lang);
  assert_eq!(_res, css);
  println!("{:?}", start.elapsed());

  // println!("{:#?}", lang);
  let start = Instant::now();
  let mut string = String::with_capacity(0);
  let mut id = 0;
  stringify(&lang, &mut id);
  // assert_eq!(_res, css);
  println!("{:?}", id);
  println!("1 {:?}", start.elapsed());
  // println!("{}", string);
  let start = Instant::now();
  let mut string = String::with_capacity(0);
  let mut id = 0;
  stringify2(lang, &mut id);
  // assert_eq!(_res, css);
  println!("{:?}", id);
  println!("2 {:?}", start.elapsed());
}

fn stringify(root: &SyntaxNode, count: &mut u32) {
  root.children().for_each(|n| {
    *count += 1;
    stringify(&n, count);
  });
}

fn stringify2(root: SyntaxNode, count: &mut u32) {
  root.preorder().for_each(|e| match e {
    WalkEvent::Enter(n) => {
      *count += 1;
    }
    WalkEvent::Leave(_) => {}
  });
}
