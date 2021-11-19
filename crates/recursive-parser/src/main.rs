use std::time::Instant;

use mimalloc_rust::*;
use recursive_parser::{
  parser::{self, Rule, RuleOrAtRuleOrDecl},
  AstPrinter,
};

// #[global_allocator]
// static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let str = include_str!("../../../assets/bootstrap.css");
  let css = r#"
  .test {
      width: 100px;
  }
  @media {
    .test result result{
      height: 100px test test;
    }
    {
      height: 100px;
    }
  }
  "#;
  let start = Instant::now();
  let mut parser = parser::Parser::new(css);
  let mut _root = parser.parse();
  println!("{:?}", start.elapsed());
  AstPrinter::default().print(&_root);
}
