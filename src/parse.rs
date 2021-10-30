use crate::input::{Input, ProcessOptions};
use crate::parser::Parser;

pub fn parse(css: &str, opts: Option<ProcessOptions>) {
  let input = Input::new(css.to_owned(), opts);
  let mut parser = Parser::new(&input);
  parser.parse();
}
