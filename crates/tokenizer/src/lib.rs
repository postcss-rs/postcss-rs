pub mod input;
mod tokenizer;

pub mod ref_ring;
pub use crate::tokenizer::*;

pub mod list;

/// **Not recommend**   
/// this method is a allocated version of tokenize which is inefficient, useful when debug.
/// almost three times slower than a non allocated on demand tokenize (just use )
pub fn tokenize(input: &str) -> Vec<Token> {
  let mut res = vec![];
  let tokenizer = Tokenizer::new(input, false);
  while !tokenizer.end_of_file() {
    res.push(tokenizer.next_token(true));
  }
  res
}
