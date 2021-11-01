use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use postcss::input::Input;
use postcss::tokenizer::{Token, Tokenizer};

fn main() {
  let css = fs::read_to_string("assets/bootstrap-reboot.css").unwrap();
  let start = Instant::now();
  let input = Input::new(&css, None);
  let processor = Tokenizer::new(input, false);
  while !processor.end_of_file() {
    processor.next_token(false);
  }
  let end = start.elapsed();
  println!("rust: tokenizer/small(7K)\t: {:?}", end);
  // let mut set = HashSet::new();
  let css = fs::read_to_string("assets/bootstrap.css").unwrap();
  let start = Instant::now();
  let input = Input::new(&css, None);
  let processor = Tokenizer::new(input, false);
  while !processor.end_of_file() {
    let token = processor.next_token(false);
    println!("{}", std::mem::size_of_val(&token));
  }
  let end = start.elapsed();
  println!("rust: tokenizer/small(201K)\t: {:?}", end);
  // println!("{:?}", set);

  // let input = Input::new("/* hello */\n.cls { font-size: 16px; } .c {".to_string(), None);
  // let mut processor = Tokenizer::new(input, true);
  // println!("{:?}", processor.next_token(false));
  // println!("{:?}", processor.next_token(false));
  // println!("{:?}", processor.next_token(false));
  // println!("{:?}", processor.next_token(false));
  // println!("{:?}", processor.next_token(false));
  // println!("{:?}", processor.next_token(false));
  // println!("{:?}", processor.next_token(false));
}
