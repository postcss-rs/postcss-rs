use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::fs::read_to_string;
use tokenizer::input::Input;
use tokenizer::Tokenizer;

#[inline]
fn tokenize<'a>(css: &'a str, ignore_errors: bool) {
  let input: Input<'a> = Input::new(css, None);
  let processor: Tokenizer<'a> = Tokenizer::new(input.css, ignore_errors);
  while !processor.end_of_file() {
    processor.next_token(false);
  }
}

fn tokenize_bench(c: &mut Criterion) {
  let file_list = [
    ("tailwind-components.css", "2.8K"),
    ("bootstrap-reboot.css", "7.4K"),
    ("bootstrap-grid.css", "71K"),
    ("bootstrap.css", "201K"),
    ("tailwind.css", "3.5M"),
    ("tailwind-dark.css", "5.8M"),
  ];

  let mut group = c.benchmark_group("tokenize");
  for (file, size) in file_list {
    let css: &str = &read_to_string(format!("../../assets/{}", file)).unwrap();
    group.bench_with_input(BenchmarkId::from_parameter(format!("{}({})", file, size)), css, |b, css| {
      b.iter(|| tokenize(css, false));
    });
  }
  group.finish();
}

criterion_group!(benches, tokenize_bench);
criterion_main!(benches);
