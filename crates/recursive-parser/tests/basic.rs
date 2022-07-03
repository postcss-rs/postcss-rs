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
      "utf8.css",
      "bootstrap.css",
      "bootstrap.rtl.css",
    ];
    for file_name in file_list {
      let file = read_to_string(format!("../../assets/{}", file_name))?;
      dbg!(&file_name);
      let expected_ast_path = format!(
        "./tests/fixtures/{}.ast",
        file_name.rsplit_once('.').unwrap().0
      );
      let expected_ast = read_to_string(expected_ast_path)?;
      let parser = Parser::new(&file);
      let root = parser.parse().unwrap();
      let mut printer = AstPrinter::new(0, WrapString::default());
      printer.print(&root)?;
      let ast = printer.result().0;
      similar_asserts::assert_str_eq!(ast, expected_ast);
    }

    Ok(())
  }

  #[test]
  fn official_integration_test_css() -> Result<()> {
    let file_list = [
      "apply.css",
      "at-rule-brackets.css",
      // "atrule-decls.css",
      "atrule-empty.css",
      "atrule-no-params.css",
      // "atrule-no-space.css",
      "atrule-params.css",
      "atrule-rules.css",
      "between.css",
      "colon-selector.css",
      // "comments.css",
      // "custom-properties.css",
      "decls.css",
      // "empty.css",
      // "escape.css",
      // "extends.css",
      "function.css",
      "ie-progid.css",
      // "important.css",
      // "inside.css",
      "no-selector.css",
      // "prop.css",
      "quotes.css",
      "raw-decl.css",
      // "rule-at.css",
      "rule-no-semicolon.css",
      "selector.css",
      "semicolons.css",
      "tab.css",
    ];
    for file_name in file_list {
      dbg!(format!("./tests/official-cases/{}", file_name));

      let file = read_to_string(format!("./tests/official-cases/{}", file_name))?;
      let expected_ast_path = format!(
        "./tests/official-cases/{}.ast",
        file_name.rsplit_once('.').unwrap().0
      );
      let expected_ast = read_to_string(expected_ast_path)?;
      let parser = Parser::new(&file);
      let root = parser.parse().unwrap();
      let mut printer = AstPrinter::new(0, WrapString::default());
      printer.print(&root)?;
      let ast = printer.result().0;
      similar_asserts::assert_str_eq!(ast, expected_ast);
    }
    Ok(())
  }
}
