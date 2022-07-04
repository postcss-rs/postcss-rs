// use std::env::current_exe;
// use std::fs::read_to_string;
use std::time::Instant;
use tokenizer::tokenize;
fn main() {
  let source = include_str!("../../../assets/bootstrap.css");
  let start = Instant::now();
  for _ in 0..100 {
    tokenize(source);
  }
  println!("{:?}", start.elapsed());
  // let vec = vec![
  //   b'\t', b'\n', b'\r', b' ', b'"', b'#', b'\'', b'(', b')', b'/', b';', b'[', b'\\', b']', b'{',
  //   b'}',
  // ];
  //   '\t', '\n', '\u{c}', '\r', ' ', '!', '"', '#', '\'', '(', ')', ':', ';', '@', '[', '\\', ']',
  //   '{', '}', '/',
  // ];
  // let file_list = [
  //   // ("tailwind-components.css", "2.8K"),
  //   // ("bootstrap-reboot.css", "7.4K"),
  //   // ("bootstrap-grid.css", "71K"),
  //   ("bootstrap.css", "201K"),
  //   // ("tailwind.css", "3.5M"),
  //   // ("tailwind-dark.css", "5.8M"),
  // ];

  // let assets_path = get_assets_path();

  // for (file, size) in file_list {
  //   let css: String = read_to_string(format!("{}/{}", assets_path, file)).unwrap();
  //   let mut vec = Vec::default();
  //   let start = Instant::now();
  //   let processor = Tokenizer::new(&css, false);
  //   while !processor.end_of_file() {
  //     vec.push(processor.next_token(false));
  //   }
  //   let end = start.elapsed();
  //   println!("rust: tokenizer/{}({}): {:?}", file, size, end);
  // }
}

// fn get_assets_path() -> String {
//   let mut path = current_exe().unwrap();
//   path.push("../../../assets");
//   path.canonicalize().unwrap().to_str().unwrap().to_string()
// }
