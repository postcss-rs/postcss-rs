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

fn from_offset_bench(c: &mut Criterion) {
  c.bench_function("from_offset/small(7K)", |b| {
    b.iter_with_large_drop(|| {
      let mut tokenizer = Tokenizer::new(SMALL_CSS_FILE, false);
      tokenizer.from_offset(7 * 1024);
    });
  });

  c.bench_function("from_offset/large(201K)", |b| {
    b.iter_with_large_drop(|| {
      let mut tokenizer = Tokenizer::new(LARGE_CSS_FILE, false);
      tokenizer.from_offset(201 * 1024);
    });
  });
}

criterion_group!(benches, tokenize_bench, from_offset_bench);
criterion_main!(benches);
