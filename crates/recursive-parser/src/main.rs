use std::{borrow::Cow, ops::Add, os::raw::c_long, time::Instant};

use mimalloc_rust::*;
use recursive_parser::{
  parser::{self, Rule, RuleOrAtRuleOrDecl},
  AstPrinter,
};
use tokenizer::{tokenize, Tokenizer};

// #[global_allocator]
// static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let str = include_str!("../../../assets/bootstrap.css");
  let css = r#".test {
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
  let css_test = r#":root {
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
  let css_test2 = r#"
a {
	color: black
}

"#;
  // let tokens = tokenize(css_test2);
  //  println!("{:#?}", tokens);

  let start = Instant::now();
  let mut parser = parser::Parser::new(css_test2);
  let mut _root = parser.parse();
  println!("{:?}", start.elapsed());
  AstPrinter::default().print(&_root);
}
