use node::Node;
use tokenizer::input::{Input, ProcessOptions};
// use node::Node;
use crate::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse(css: &str, opts: Option<ProcessOptions>) -> Rc<RefCell<Node>> {
  let input = Input::new(css, opts);
  let mut parser = Parser::new(input);
  parser.parse();
  parser.root
}
