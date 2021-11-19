use std::time::Instant;

use mimalloc_rust::*;
use recursive_parser::{AstPrinter, parser::{self, Rule, RuleOrAtRuleOrDecl}};

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let str = include_str!("../../../assets/bootstrap.css");
  let css = r#"
  .test {
      width: 100px;
  }
  "#;
  println!("{}", css.len());;
  let start = Instant::now();
  let mut parser = parser::Parser::new(css);
  let mut _root = parser.parse();
  AstPrinter::default().print(&_root);
}
