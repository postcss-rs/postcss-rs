// use mimalloc_rust::*;
use recursive_parser::parse;
use std::fs::read_to_string;
use std::time::Instant;

// #[global_allocator]
// static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

fn main() {
  let file_list = [
    ("tailwind-components.css", "2.8K"),
    ("bootstrap-reboot.css", "7.4K"),
    ("bootstrap-grid.css", "71K"),
    ("bootstrap.css", "201K"),
    ("tailwind.css", "3.5M"),
    ("tailwind-dark.css", "5.8M"),
  ];

  for (file, size) in file_list {
    let css: &str = &read_to_string(format!("../../assets/{}", file)).unwrap();
    let start = Instant::now();
    parse(css, Some(file));
    let end = start.elapsed();
    println!("rust: tokenizer/{}({}): {:?}", file, size, end);
  }
}
