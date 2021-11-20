use std::time::Instant;

use mimalloc_rust::*;
use recursive_parser::{
  parser::{self, Rule, RuleOrAtRuleOrDecl},
  AstPrinter,
};

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
  /* 我的 */
@supports (--element(".minwidth", { "minWidth": 300 })) {
  [--self] {
    background: greenyellow;
  }
}

@supports ({"example": 1}) {
  * { background: red; }
}

@supports (("example": 1)) {
  * { background: red; }
}

@supports (func("example": 1)) {
  * { background: red; }
}

@supports (--var) {
  * { background: red; }
}

@supports ([]) {
  * { background: red; }
}

@supports ([color: red]) {
  * { background: red; }
}

@supports ([[[[[{ --func(color: { red }) }]]]]]) {
  * { background: red; }
}

  

  "#;
  let start = Instant::now();
  let mut parser = parser::Parser::new(css_test2);
  let mut _root = parser.parse();
  println!("{:?}", start.elapsed());
// AstPrinter::default().print(&_root);
}
