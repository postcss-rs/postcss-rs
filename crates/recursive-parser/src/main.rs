use std::time::Instant;

use mimalloc_rust::*;
use recursive_parser::parser;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let str = include_str!("../../../assets/bootstrap.css");
  let start = Instant::now();
  let mut parser = parser::Parser::new(str);
  let _root = parser.parse();
  println!("{:?}", start.elapsed());
}
