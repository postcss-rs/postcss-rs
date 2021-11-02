use crate::input::{Input, ProcessOptions};
use crate::node::Node;
use crate::parser::Parser;
use std::cell::RefCell;

pub fn parse(css: &str, opts: Option<ProcessOptions>) -> RefCell<Node> {
  let input = Input::new(css, opts);
  let mut parser = Parser::new(input);
  parser.parse();
  parser.root
}
