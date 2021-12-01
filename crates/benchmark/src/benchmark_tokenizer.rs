use mimalloc_rust::*;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;
use tokenizer::Tokenizer;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

fn main() {
  let file = env::args().nth(1).unwrap();
  let css: &str = &read_to_string(format!("assets/{}", file)).unwrap();
  let start = Instant::now();
  let processor = Tokenizer::new(css, false);
  while !processor.end_of_file() {
    processor.next_token(false);
  }
  let end = start.elapsed();
  print!("{}", end.as_nanos());
}
