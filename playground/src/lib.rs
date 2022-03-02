mod utils;

use recursive_parser::{AstPrinter, WrapString};
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn ast(source: String) -> String {
  let parser = recursive_parser::parser::Parser::new(&source);

  match parser.parse() {
    Ok(root) => {
      let mut printer = AstPrinter::new(0, WrapString::default());
      printer.print(&root).unwrap();
      let ast = printer.result().0;
      return ast;
    }
    Err(_) => return "".to_string(),
  };
}
