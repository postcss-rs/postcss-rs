use criterion::{criterion_group, criterion_main, Criterion};
use recursive_parser::parser::{Parser, Root};

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
const SMALL_CSS_FILE: &str = include_str!("../../../assets/bootstrap-reboot.css");
const LARGE_CSS_FILE: &str = include_str!("../../../assets/bootstrap.css");

fn parse<'a>(css: &'a str) -> Root {
  let parser = Parser::new(css);
  parser.parse().unwrap()
}

fn tokenize_bench(c: &mut Criterion) {
  c.bench_function("recursive_parser/small(7K)", |b| {
    b.iter_with_large_drop(|| parse(SMALL_CSS_FILE));
  });
  c.bench_function("recursive_parser/large(201K)", |b| {
    b.iter_with_large_drop(|| parse(LARGE_CSS_FILE));
  });
}

criterion_group!(benches, tokenize_bench);
criterion_main!(benches);
