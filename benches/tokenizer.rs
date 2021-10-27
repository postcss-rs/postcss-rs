use criterion::{black_box, criterion_group, criterion_main, Criterion};
use postcss::{
  input::Input,
  tokenizer::{Token, Tokenizer},
};

const SMALL_CSS_FILE: &str = include_str!("../assets/bootstrap-reboot.css");
const LARGE_CSS_FILE: &str = include_str!("../assets/bootstrap.css");

fn tokenize(css: &str, ignore_errors: bool) -> Vec<Token> {
  let input = Input::new(css.to_string(), None);
  let mut processor = Tokenizer::new(&input, ignore_errors);
  let mut tokens = vec![];
  while !processor.end_of_file() {
    tokens.push(processor.next_token(false))
  }
  return tokens;
}

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("small css file 7K", |b| {
    b.iter(|| {
      tokenize(SMALL_CSS_FILE, false);
    });
  });
  c.bench_function("large css file 201K", |b| {
    b.iter(|| {
      tokenize(LARGE_CSS_FILE, false);
    });
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
