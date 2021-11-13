// use rowan_parser::parser::Parser;

use std::time::Instant;

use cssparser::{BasicParseError, Parser as CssParser, ParserInput, Token};
fn main() {
  let source_code = include_str!("../../../assets/test.css");
  // let parser = CssParser::new(&mut ParserInput::new(source_code));
  let start = Instant::now();
  parse_and_serialize(source_code);
  println!("{:?}", start.elapsed());
}

fn parse_and_serialize(input: &str) -> () {
  let mut input = ParserInput::new(input);
  let mut parser = CssParser::new(&mut input);
  // let mut serialization = String::new();
  loop {
    let loc = parser.position();
    let token = parser.next_including_whitespace_and_comments();
    let len = token.as_ref().unwrap();
    println!("loc: {:?}, {:?}", loc, token);
    match token {
      Ok(token) => match token {
        Token::Ident(_) => (),
        Token::AtKeyword(_) => (),
        Token::Hash(_) => (),
        Token::IDHash(_) => (),
        Token::QuotedString(_) => (),
        Token::UnquotedUrl(_) => (),
        Token::Delim(_) => (),
        Token::Number {
          has_sign,
          value,
          int_value,
        } => (),
        Token::Percentage {
          has_sign,
          unit_value,
          int_value,
        } => (),
        Token::Dimension {
          has_sign,
          value,
          int_value,
          unit,
        } => (),
        Token::WhiteSpace(_) => (),
        Token::Comment(_) => (),
        Token::Colon => (),
        Token::Semicolon => (),
        Token::Comma => (),
        Token::IncludeMatch => (),
        Token::DashMatch => (),
        Token::PrefixMatch => (),
        Token::SuffixMatch => (),
        Token::SubstringMatch => (),
        Token::CDO => (),
        Token::CDC => (),
        Token::Function(a) => (),
        Token::ParenthesisBlock => (),
        Token::SquareBracketBlock => (),
        Token::CurlyBracketBlock => {}
        Token::BadUrl(_) => (),
        Token::BadString(_) => (),
        Token::CloseParenthesis => (),
        Token::CloseSquareBracket => (),
        Token::CloseCurlyBracket => (),
      },
      Err(err) => break,
    };
  }
}
// fn main() {
//   let mut parser = Parser::new("[ ] [");
//   println!("{:?}", parser.peek());;
// }
