use criterion::{criterion_group, criterion_main, Criterion};
use rowan_parser::{parser, syntax::SyntaxNode};
const SMALL_CSS_FILE: &str = include_str!("../../../assets/bootstrap-reboot.css");
const LARGE_CSS_FILE: &str = include_str!("../../../assets/bootstrap.css");

fn parse<'a>(css: &'a str) {
  let parser = parser::Parser::new(css);
  let node = parser.parse().green_node;
  let _root = SyntaxNode::new_root(node);
}

fn tokenize_bench(c: &mut Criterion) {
  c.bench_function("parser/small(7K)", |b| {
    b.iter_with_large_drop(|| parse(SMALL_CSS_FILE));
  });
  c.bench_function("parser/large(201K)", |b| {
    b.iter_with_large_drop(|| parse(LARGE_CSS_FILE));
  });
}

criterion_group!(benches, tokenize_bench);
criterion_main!(benches);
