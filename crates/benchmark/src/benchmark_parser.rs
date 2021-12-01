use mimalloc_rust::*;
use recursive_parser::parse;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

fn main() {
  let file = env::args().nth(1).unwrap();
  let css: &str = &read_to_string(format!("assets/{}", file)).unwrap();
  let start = Instant::now();
  parse(css, None);
  let end = start.elapsed();
  print!("{}", end.as_nanos());
}
