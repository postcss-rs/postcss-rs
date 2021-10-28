use criterion::{criterion_group, criterion_main, Criterion};
use postcss::input::Input;
use postcss::tokenizer::{Token, Tokenizer};

const SMALL_CSS_FILE: &str = include_str!("../assets/bootstrap-reboot.css");
const LARGE_CSS_FILE: &str = include_str!("../assets/bootstrap.css");

fn tokenize(css: &str, ignore_errors: bool) {
  let input = Input::new(css.to_string(), None);
  let mut processor = Tokenizer::new(&input, ignore_errors);
  while !processor.end_of_file() {
    processor.next_token(false);
  }
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
