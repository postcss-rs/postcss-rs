use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::clone::Clone;
use std::cmp::Eq;
use std::cmp::PartialEq;

use crate::input::Input;

const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';
const BACKSLASH: char = '\\';
const SLASH: char = '/';
const NEWLINE: char = '\n';
const SPACE: char = ' ';
const FEED: char = '\u{12}'; // \f
const TAB: char = '\t';
const CR: char = '\r';
const OPEN_SQUARE: char = '[';
const CLOSE_SQUARE: char = ']';
const OPEN_PARENTHESES: char = '(';
const CLOSE_PARENTHESES: char = ')';
const OPEN_CURLY: char = '{';
const CLOSE_CURLY: char = '}';
const SEMICOLON: char = ';';
const ASTERISK: char = '*';
const COLON: char = ':';
const AT: char = '@';

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token(
  pub &'static str,
  pub String,
  pub Option<usize>,
  pub Option<usize>,
);

impl Token {
  pub fn new(kind: &'static str, content: &str, pos: Option<usize>, next: Option<usize>) -> Token {
    Token(kind, content.to_string(), pos, next)
  }
}

#[derive(Debug)]
pub struct Tokenizer {
  css: String,
  ignore: bool,
  quote: char,
  content: String,
  escape: bool,
  escaped: bool,
  escape_pos: usize,
  prev: String,
  n: char,
  current_token: Token,
  length: usize,
  pos: usize,
  next: usize,
  buffer: Vec<Token>,
  returned: Vec<Token>,
}

impl Tokenizer {
  pub fn new(input: Input, ignore_errors: bool) -> Tokenizer {
    let length = input.css.chars().count();
    Tokenizer {
      css: input.css,
      ignore: ignore_errors,
      next: 0,
      quote: '\0',
      content: String::new(),
      escape: false,
      escaped: false,
      escape_pos: 0,
      prev: String::new(),
      n: '\0',
      current_token: Token("", String::new(), None, None),
      length,
      pos: 0,
      buffer: vec![],
      returned: vec![],
    }
  }

  #[inline]
  fn push(&mut self, t: Token) {
    self.buffer.push(t);
  }

  pub fn position(&self) -> usize {
    self.pos
  }

  pub fn unclosed(&self, what: &str) {
    panic!("Unclosed {} {}", what, self.pos);
  }

  pub fn end_of_file(&self) -> bool {
    self.returned.is_empty() && self.pos >= self.length
  }

  pub fn back(&mut self, token: Token) {
    self.returned.push(token);
  }

  pub fn next_token(&mut self, ignore_unclosed: bool) -> Option<Token> {
    lazy_static! {
      static ref RE_AT_END: Regex = Regex::new(r##"[\t\n\u{12}\r "#'()/;\[\\\]{}]"##).unwrap();
      static ref RE_WORD_END: Regex =
        Regex::new(r##"[\t\n\u{12}\r !"#'():;@\[\\\]{}]|/(?=\*)"##).unwrap();
      static ref RE_BAD_BRACKET: Regex = Regex::new(r#".[\n"'(/\\]"#).unwrap();
      static ref RE_HEX_ESCAPE: Regex = Regex::new(r"[\da-f]").unwrap();
    }

    if !self.returned.is_empty() {
      return self.returned.pop();
    }

    if self.pos >= self.length {
      return None;
    }

    let mut code = char_code_at(&self.css, self.pos);

    match code {
      NEWLINE | SPACE | TAB | CR | FEED => {
        self.next = self.pos;
        loop {
          self.next += 1;
          code = char_code_at(&self.css, self.next);
          if !(code == SPACE || code == NEWLINE || code == TAB || code == FEED) {
            break;
          }
        }

        self.current_token = Token(
          "space",
          self.css.as_str()[self.pos..self.next].to_string(),
          None,
          None,
        );

        self.pos = self.next - 1;
      }
      OPEN_SQUARE => {
        self.current_token = Token("[", "[".to_string(), Some(self.pos), None);
      }
      CLOSE_SQUARE => {
        self.current_token = Token("]", "]".to_string(), Some(self.pos), None);
      }
      OPEN_CURLY => {
        self.current_token = Token("{", "{".to_string(), Some(self.pos), None);
      }
      CLOSE_CURLY => {
        self.current_token = Token("}", "}".to_string(), Some(self.pos), None);
      }
      COLON => {
        self.current_token = Token(":", ":".to_string(), Some(self.pos), None);
      }
      SEMICOLON => {
        self.current_token = Token(";", ";".to_string(), Some(self.pos), None);
      }
      CLOSE_PARENTHESES => {
        self.current_token = Token(")", ")".to_string(), Some(self.pos), None);
      }
      OPEN_PARENTHESES => {
        self.prev = match self.buffer.pop() {
          Some(b) => b.1,
          None => String::new(),
        };
        let n = char_code_at(&self.css, self.pos + 1);
        if self.prev == "url"
          && n != SINGLE_QUOTE
          && n != DOUBLE_QUOTE
          && n != SPACE
          && n != NEWLINE
          && n != TAB
          && n != FEED
          && n != CR
        {
          self.next = self.pos;
          loop {
            self.escaped = false;
            match index_of(&self.css, ")", self.next + 1) {
              Some(i) => {
                self.next = i;
              }
              None => {
                if self.ignore || ignore_unclosed {
                  self.next = self.pos;
                  break;
                } else {
                  self.unclosed("bracket")
                }
              }
            }
            self.escape_pos = self.next;
            while char_code_at(&self.css, self.escape_pos - 1) == BACKSLASH {
              self.escape_pos -= 1;
              self.escaped = !self.escaped;
            }

            if !self.escaped {
              break;
            }
          }

          self.current_token = Token(
            "brackets",
            sub_string(&self.css, self.pos, self.next + 1).to_string(),
            Some(self.pos),
            Some(self.next),
          );

          self.pos = self.next;
        } else {
          match index_of_char(&self.css, ')', self.pos + 1) {
            Some(next) => {
              self.next = next + self.pos + 1;
              let content = &self.css[self.pos..next + 1];

              if RE_BAD_BRACKET.is_match(content).unwrap_or(false) {
                self.current_token = Token("(", "(".to_string(), Some(self.pos), None);
              } else {
                self.current_token =
                  Token("brackets", content.to_string(), Some(self.pos), Some(next));
                self.pos = next;
              }
            }
            None => {
              self.current_token = Token("(", "(".to_string(), Some(self.pos), None);
            }
          };
        }
      }
      SINGLE_QUOTE | DOUBLE_QUOTE => {
        self.quote = if code == SINGLE_QUOTE { '\'' } else { '"' };
        self.next = self.pos;
        loop {
          self.escaped = false;
          match index_of_char(&self.css, self.quote, self.next + 1) {
            Some(i) => {
              self.next = i;
            }
            None => {
              if self.ignore || ignore_unclosed {
                self.next = self.pos + 1;
                break;
              } else {
                self.unclosed("string")
              }
            }
          }
          self.escape_pos = self.next;
          while char_code_at(&self.css, self.escape_pos - 1) == BACKSLASH {
            self.escape_pos -= 1;
            self.escaped = !self.escaped;
          }

          if !self.escaped {
            break;
          }
        }

        self.current_token = Token(
          "string",
          sub_string(&self.css, self.pos, self.next + 1).to_string(),
          Some(self.pos),
          Some(self.next),
        );
        self.pos = self.next;
      }
      AT => {
        self.next = match RE_AT_END.find(&self.css.as_str()[self.pos + 1..]).unwrap() {
          Some(mat) => self.pos + 1 + mat.end() - 2,
          None => self.length - 1,
        };
        self.current_token = Token(
          "at-word",
          sub_string(&self.css, self.pos, self.next + 1).to_string(),
          Some(self.pos),
          Some(self.next),
        );
        self.pos = self.next;
      }
      BACKSLASH => {
        self.next = self.pos;
        self.escape = true;
        while char_code_at(&self.css, self.next + 1) == BACKSLASH {
          self.next += 1;
          self.escape = !self.escape;
        }
        code = char_code_at(&self.css, self.next + 1);
        if self.escape
          && code != SLASH
          && code != SPACE
          && code != NEWLINE
          && code != TAB
          && code != CR
          && code != FEED
        {
          self.next += 1;
          if RE_HEX_ESCAPE
            .is_match(sub_string(&self.css, self.next, self.next + 1))
            .unwrap_or(false)
          {
            while RE_HEX_ESCAPE
              .is_match(sub_string(&self.css, self.next + 1, self.next + 2))
              .unwrap_or(false)
            {
              self.next += 1;
            }
            if char_code_at(&self.css, self.next + 1) == SPACE {
              self.next += 1;
            }
          }
        }

        self.current_token = Token(
          "word",
          sub_string(&self.css, self.pos, self.next + 1).to_string(),
          Some(self.pos),
          Some(self.next),
        );
        self.pos = self.next;
      }
      _ => {
        if code == SLASH && char_code_at(&self.css, self.pos + 1) == ASTERISK {
          match index_of(&self.css, "*/", self.pos + 2) {
            Some(i) => {
              self.next = i + 1;
            }
            None => {
              if self.ignore || ignore_unclosed {
                self.next = self.length;
              } else {
                self.unclosed("comment")
              }
            }
          }

          self.current_token = Token(
            "comment",
            sub_string(&self.css, self.pos, self.next + 1).to_string(),
            Some(self.pos),
            Some(self.next),
          );
        } else {
          self.next = match RE_WORD_END
            .find(&self.css.as_str()[self.pos + 1..])
            .unwrap()
          {
            Some(mat) => self.pos + mat.end() - 1,
            None => self.length - 1,
          };
          self.current_token = Token(
            "word",
            sub_string(&self.css, self.pos, self.next + 1).to_string(),
            Some(self.pos),
            Some(self.next),
          );
          self.push(self.current_token.clone());
        }

        self.pos = self.next;
      }
    }

    self.pos += 1;
    Some(self.current_token.clone())
  }
}

#[inline]
fn index_of(value: &str, search_value: &str, from_index: usize) -> Option<usize> {
  let (_, last) = value.split_at(from_index);
  last.find(search_value).map(|v| v + from_index)
}

#[inline]
fn index_of_char(value: &str, search_value: char, from_index: usize) -> Option<usize> {
  let (_, last) = value.split_at(from_index);
  last.find(search_value).map(|v| v + from_index)
}

#[inline]
fn sub_string(s: &str, start: usize, end: usize) -> &str {
  if end + 1 > s.len() {
    &s[start..]
  } else {
    &s[start..end]
  }
}

#[inline]
fn char_code_at(s: &str, n: usize) -> char {
  if n >= s.len() {
    '\0'
  } else {
    s.as_bytes()[n] as char
  }
  // s.chars().nth(n).unwrap_or('\0')
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_char_code_at() {
    let s = "0123456789abc";
    assert_eq!(char_code_at(s, 0), '0');
    assert_eq!(char_code_at(s, 1), '1');
    assert_eq!(char_code_at(s, 100), '\0');
  }
}
