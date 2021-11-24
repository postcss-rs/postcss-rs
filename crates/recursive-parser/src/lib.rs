mod ast_util;

pub mod parser;
pub mod syntax;
pub mod visitor;

pub use ast_util::*;

use codespan_reporting::term;
use parser::Root;
pub mod error;

pub fn parse<'a>(input: &'a str, file_name: Option<&'a str>) -> Root<'a> {
  let file_name = file_name.unwrap_or("default.css");
  let parser = parser::Parser::new(input);
  match parser.parse() {
    Ok(root) => root,
    Err(err) => match err {
      error::PostcssError::ParseError(msg, start, end) => {
        use codespan_reporting::diagnostic::{Diagnostic, Label};
        use codespan_reporting::files::SimpleFiles;
        use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
        let mut files = SimpleFiles::new();

        let file_id = files.add(file_name, input);

        let diagnostic = Diagnostic::error()
          .with_message("postcss parse error")
          .with_labels(vec![
            Label::primary(file_id, start..end).with_message("css parse error")
          ])
          .with_notes(vec![msg]);

        // We now set up the writer and configuration, and then finally render the
        // diagnostic to standard error.

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
        panic!()
      }
      error::PostcssError::Unknown => todo!(),
    },
  }
}
