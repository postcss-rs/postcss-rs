use std::fs;
use std::time::Instant;

use tokenizer::input::Input;
use tokenizer::Tokenizer;

fn main() {
  let css = fs::read_to_string("assets/bootstrap-reboot.css").unwrap();
  let start = Instant::now();
  let input = Input::new(&css, None);
  let processor = Tokenizer::new(input.css, false);
  while !processor.end_of_file() {
    processor.next_token(false);
  }
  let end = start.elapsed();
  println!("rust: tokenizer/small(7K)\t\t: {:?}", end);

  let css = fs::read_to_string("assets/bootstrap.css").unwrap();
  let start = Instant::now();
  let input = Input::new(&css, None);
  let processor = Tokenizer::new(input.css, false);
  while !processor.end_of_file() {
    processor.next_token(false);
  }
  let end = start.elapsed();
  println!("rust: tokenizer/fairly_large(201K)\t: {:?}", end);
  println!("{:?}", '\u{c}' as u32);
  println!("{:?}", '\r' as u32);
}
