use criterion::{criterion_group, criterion_main, Criterion};
use tokenizer::input::Input;
use tokenizer::{Token, Tokenizer};

const SMALL_CSS_FILE: &str = include_str!("../../../assets/bootstrap-reboot.css");
const LARGE_CSS_FILE: &str = include_str!("../../../assets/bootstrap.css");

fn tokenize<'a>(css: &'a str, ignore_errors: bool) -> Vec<Token<'a>> {
  let input: Input<'a> = Input::new(css, None);
  let processor: Tokenizer<'a> = Tokenizer::new(input.css, ignore_errors);
  let mut tokens = vec![];
  while !processor.end_of_file() {
    tokens.push(processor.next_token(false))
  }
  tokens
}

fn tokenize_bench(c: &mut Criterion) {
  c.bench_function("tokenizer/small(7K)", |b| {
    b.iter_with_large_drop(|| tokenize(SMALL_CSS_FILE, false));
  });
  c.bench_function("tokenizer/large(201K)", |b| {
    b.iter_with_large_drop(|| tokenize(LARGE_CSS_FILE, false));
  });
}

criterion_group!(benches, tokenize_bench);
criterion_main!(benches);
