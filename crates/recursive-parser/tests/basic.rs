use anyhow::Result;
#[cfg(test)]
mod test_ast {
  use std::fs::read_to_string;

  use recursive_parser::{parser::Parser, AstPrinter, WrapString};

  use super::*;

  #[test]
  fn integration_test_css() -> Result<()> {
    let file_list = [
      "simple.css",
      "bootstrap-reboot.css",
      "bootstrap-reboot.rtl.css",
      "bootstrap-grid.css",
      "bootstrap-grid.rtl.css",
      "bootstrap-utilities.css",
      "bootstrap-utilities.rtl.css",
    ];
    for file_name in file_list {
      let file = read_to_string(format!("../../assets/{}", file_name))?;
      dbg!(&file_name);
      let expected_ast_path = format!(
        "./tests/fixtures/{}.ast",
        file_name.rsplit_once(".").unwrap().0
      );
      let expected_ast = read_to_string(expected_ast_path)?;
      let parser = Parser::new(&file);
      let root = parser.parse();
      let mut printer = AstPrinter::new(0, WrapString::default());
      printer.print(&root)?;
      let ast = printer.result().0;
      similar_asserts::assert_str_eq!(ast, expected_ast);
    }

    Ok(())
  }
}
