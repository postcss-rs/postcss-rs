#![feature(path_file_prefix)]
use mimalloc_rust::*;
use recursive_parser::parser::Parser;
use recursive_parser::{AstPrinter, WrapString};
use std::path::PathBuf;
use std::time::Instant;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let _file = include_str!("../../../assets/tailwind.css");
  let _css = r#".test {
      width: 100px
  }
  @media {
    .test result result{
      height: 100px test test;
    }
    {
      height: 100px;
    }
  }
  "#;
  let _css1 = r#":root {
    --zero-size: {
      width: 0;
      height: 0;
    };
    --small-icon: {
      width: 16px;
      height: 16px;
    }
    ;
  }"#;
  let css2 = r#"
a {
	color: black
}

"#;
  // let tokens = tokenize(css2);
  //  println!("{:#?}", tokens);

  let start = Instant::now();
  let parser = Parser::new(css2);
  let root = parser.parse();
  println!("{:?}", start.elapsed());
  let stdout = std::io::stdout();
  AstPrinter::new(0, stdout).print(&root).unwrap();
  let res = PathBuf::from("test.css");
  println!("{:?}", res.file_prefix());
  let parser = Parser::new(css2);
  let root = parser.parse();
  let mut printer = AstPrinter::new(0, WrapString::default());
  printer.print(&root).unwrap();
  let ast = printer.result().0;
  print!("{}", ast);
}
