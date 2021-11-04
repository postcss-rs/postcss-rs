use once_cell::sync::Lazy;
use std::fmt;

#[derive(Debug)]
pub struct FilePosition {
  url: String,
  file: Option<String>,
  source: Option<String>,
  line: u32,
  column: u32,
}

#[derive(Debug)]
pub struct Position {
  line: u32,
  col: u32,
}

impl fmt::Display for FilePosition {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{:?}:{}:{}:{}:{:?}",
      self.file, self.line, self.column, self.url, self.source
    )
  }
}

static DEFAULT_INPUT: Lazy<Input> = Lazy::new(Input::default);
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Input<'a> {
  pub css: &'a str,
  // map: PreviousMap,
  file: Option<String>,
  id: Option<String>,
  has_bom: bool,
  line: u32,
  column: u32,
}

impl<'a> Default for &'a Input<'a> {
  fn default() -> &'a Input<'a> {
    &DEFAULT_INPUT
  }
}

impl<'a> fmt::Display for Input<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{:?}:{}:{}:{}:{}",
      self.file, self.line, self.column, self.css, self.has_bom
    )
  }
}

#[derive(Debug)]
pub struct ProcessOptions {}

impl<'a> Input<'a> {
  pub fn new(css: &'a str, _opts: Option<ProcessOptions>) -> Input<'a> {
    Input {
      css,
      file: Some(String::new()),
      id: Some(String::from("123")),
      has_bom: false,
      line: 1,
      column: 1,
    }
  }

  pub fn from(&self) -> String {
    String::from("/home/ai/a.css")
  }

  pub fn origin(&self, line: u32, column: u32) -> Option<FilePosition> {
    Some(FilePosition {
      url: String::from("/home/ai/a.css"),
      file: Some(String::from("/home/ai/a.css")),
      source: Some(String::from(".className {}")),
      line,
      column,
    })
  }

  pub fn from_offset(&self, offset: u32) -> Option<Position> {
    Some(Position {
      line: 1,
      col: offset,
    })
  }
}
