use crate::input::Input;
use lazy_static::lazy_static;
use memchr::memchr;
use memchr::memmem::Finder;
use std::cell::RefCell;
use std::clone::Clone;
use std::cmp::Eq;
use std::cmp::PartialEq;

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

lazy_static! {
  static ref FINDER_END_OF_COMMENT: Finder<'static> = Finder::new("*/");
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token<'a>(
  pub &'static str,
  pub &'a str,
  pub Option<usize>,
  pub Option<usize>,
);

impl<'a> Token<'a> {
  pub fn new(
    kind: &'static str,
    content: &'a str,
    pos: Option<usize>,
    next: Option<usize>,
  ) -> Token<'a> {
    Token(kind, content, pos, next)
  }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
  css: &'a str,
  ignore: bool,
  length: usize,
  pos: RefCell<usize>,
  buffer: RefCell<Vec<&'a str>>,
  returned: RefCell<Vec<Token<'a>>>,
}

impl<'a> Tokenizer<'a> {
  pub fn new(input: Input<'a>, ignore_errors: bool) -> Tokenizer<'a> {
    let length = input.css.len();
    Tokenizer {
      css: input.css,
      ignore: ignore_errors,
      length,
      pos: RefCell::new(0),
      // buffer: Vec::with_capacity(length / 13),
      buffer: RefCell::new(vec![]),
      returned: RefCell::new(vec![]),
    }
  }

  #[inline]
  fn push(&self, t: &'a str) {
    self.buffer.borrow_mut().push(t);
  }

  pub fn position(&self) -> usize {
    *self.pos.borrow()
  }

  pub fn unclosed(&self, what: &str) {
    panic!("Unclosed {} {}", what, self.position());
  }

  pub fn end_of_file(&self) -> bool {
    self.returned.borrow().is_empty() && self.position() >= self.length
  }

  pub fn back(&self, token: Token<'a>) {
    self.returned.borrow_mut().push(token);
  }

  #[inline]
  fn pos_plus_one(&self) {
    self.pos.replace_with(|it| *it + 1);
  }

  pub fn next_token(&self, ignore_unclosed: bool) -> Token<'a> {
    if !self.returned.borrow().is_empty() {
      return self.returned.borrow_mut().pop().unwrap();
    }

    let mut code = char_code_at(self.css, self.position());

    let current_token: Token;

    match code {
      NEWLINE | SPACE | TAB | CR | FEED => {
        let mut next = self.position();
        loop {
          next += 1;
          code = char_code_at(self.css, next);
          if !(code == SPACE || code == NEWLINE || code == TAB || code == FEED) {
            break;
          }
        }

        current_token = Token("space", self.css[self.position()..next].into(), None, None);

        self.pos.replace(next);
      }
      OPEN_SQUARE => {
        current_token = Token("[", "[", Some(self.position()), None);
        self.pos_plus_one();
      }
      CLOSE_SQUARE => {
        current_token = Token("]", "]", Some(self.position()), None);
        self.pos_plus_one();
      }
      OPEN_CURLY => {
        current_token = Token("{", "{", Some(self.position()), None);
        self.pos_plus_one();
      }
      CLOSE_CURLY => {
        current_token = Token("}", "}", Some(self.position()), None);
        self.pos_plus_one();
      }
      COLON => {
        current_token = Token(":", ":", Some(self.position()), None);
        self.pos_plus_one();
      }
      SEMICOLON => {
        current_token = Token(";", ";", Some(self.position()), None);
        self.pos_plus_one();
      }
      CLOSE_PARENTHESES => {
        current_token = Token(")", ")", Some(self.position()), None);
        self.pos_plus_one();
      }
      OPEN_PARENTHESES => {
        let prev = self.buffer.borrow_mut().pop().unwrap_or("");
        let n = char_code_at(self.css, self.position() + 1);
        if prev == "url"
          && n != SINGLE_QUOTE
          && n != DOUBLE_QUOTE
          && n != SPACE
          && n != NEWLINE
          && n != TAB
          && n != FEED
          && n != CR
        {
          let mut next = self.position();
          loop {
            let mut escaped = false;
            match index_of_byte(self.css, b')', next + 1) {
              Some(i) => {
                next = i;
              }
              None => {
                if self.ignore || ignore_unclosed {
                  next = self.position();
                  break;
                } else {
                  self.unclosed("bracket")
                }
              }
            }

            let mut escape_pos = next;
            while char_code_at(self.css, escape_pos - 1) == BACKSLASH {
              escape_pos -= 1;
              escaped = !escaped;
            }

            if !escaped {
              break;
            }
          }

          current_token = Token(
            "brackets",
            sub_string(self.css, self.position(), next + 1),
            Some(self.position()),
            Some(next),
          );

          self.pos.replace(next + 1);
        } else {
          match index_of_byte(self.css, b')', self.position() + 1) {
            Some(i) => {
              let content = &self.css[self.position()..i + 1];

              if is_bad_bracket(content) {
                current_token = Token("(", "(", Some(self.position()), None);
              } else {
                current_token = Token("brackets", content, Some(self.position()), Some(i));
                self.pos.replace(i);
              }
            }
            None => {
              current_token = Token("(", "(", Some(self.position()), None);
            }
          };
          self.pos_plus_one();
        }
      }
      SINGLE_QUOTE | DOUBLE_QUOTE => {
        let quote = if code == SINGLE_QUOTE { b'\'' } else { b'"' };
        let mut next = self.position();
        loop {
          let mut escaped = false;
          match index_of_byte(self.css, quote, next + 1) {
            Some(i) => {
              next = i;
            }
            None => {
              if self.ignore || ignore_unclosed {
                next = self.position() + 1;
                break;
              } else {
                self.unclosed("string")
              }
            }
          }

          let mut escape_pos = next;
          while char_code_at(self.css, escape_pos - 1) == BACKSLASH {
            escape_pos -= 1;
            escaped = !escaped;
          }

          if !escaped {
            break;
          }
        }

        current_token = Token(
          "string",
          sub_string(self.css, self.position(), next + 1),
          Some(self.position()),
          Some(next),
        );
        self.pos.replace(next + 1);
      }
      AT => {
        let next = index_of_at_end(self.css, self.position() + 1) - 1;
        current_token = Token(
          "at-word",
          sub_string(self.css, self.position(), next + 1),
          Some(self.position()),
          Some(next),
        );
        self.pos.replace(next + 1);
      }
      BACKSLASH => {
        let mut next = self.position();
        let mut escape = true;
        while char_code_at(self.css, next + 1) == BACKSLASH {
          next += 1;
          escape = !escape;
        }
        code = char_code_at(self.css, next + 1);
        if escape
          && code != SLASH
          && code != SPACE
          && code != NEWLINE
          && code != TAB
          && code != CR
          && code != FEED
        {
          next += 1;
          if is_hex_char(self.css, next) {
            while is_hex_char(self.css, next + 1) {
              next += 1;
            }
            if char_code_at(self.css, next + 1) == SPACE {
              next += 1;
            }
          }
        }

        current_token = Token(
          "word",
          sub_string(self.css, self.position(), next + 1),
          Some(self.position()),
          Some(next),
        );
        self.pos.replace(next + 1);
      }
      _ => {
        self.pos.replace(
          if code == SLASH && char_code_at(self.css, self.position() + 1) == ASTERISK {
            let next = match index_of_end_comment(self.css, self.position() + 2) {
              Some(i) => i + 1,
              None => {
                if !self.ignore && !ignore_unclosed {
                  self.unclosed("comment");
                }
                self.length
              }
            };

            current_token = Token(
              "comment",
              sub_string(self.css, self.position(), next + 1),
              Some(self.position()),
              Some(next),
            );
            next
          } else {
            let next = index_of_word_end(self.css, self.position() + 1) - 1;
            let content = sub_string(self.css, self.position(), next + 1);
            current_token = Token("word", content, Some(self.position()), Some(next));
            self.push(content);
            next
          },
        );
        self.pos_plus_one();
      }
    }

    current_token
  }
}

#[inline]
fn index_of_end_comment(value: &str, from_index: usize) -> Option<usize> {
  let (_, last) = value.split_at(from_index);
  FINDER_END_OF_COMMENT
    .find(last.as_bytes())
    .map(|v| v + from_index)
}

#[inline]
fn index_of_byte(value: &str, search_value: u8, from_index: usize) -> Option<usize> {
  let (_, last) = value.split_at(from_index);
  memchr(search_value, last.as_bytes()).map(|v| v + from_index)
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
}

#[inline]
fn is_hex_char(s: &str, n: usize) -> bool {
  if n >= s.len() {
    return false;
  }

  matches!(s.as_bytes()[n], b'A'..=b'F' | b'a'..=b'f' | b'0'..=b'9')
}

#[inline]
fn is_bad_bracket(s: &str) -> bool {
  let bytes = s.as_bytes();
  for i in 1..bytes.len() {
    match bytes[i] as char {
      '\n' | '"' | '\'' | '(' | '/' | '\\' => {
        return true;
      }
      _ => continue,
    };
  }
  false
}

#[inline]
fn index_of_at_end(s: &str, start: usize) -> usize {
  let bytes = s.as_bytes();
  let mut i = start;
  let len = bytes.len();

  while i < len {
    match bytes[i] as char {
      '\t' | '\n' | '\u{12}' | '\r' | ' ' | '"' | '#' | '\'' | '(' | ')' | '/' | ';' | '['
      | '\\' | ']' | '{' | '}' => {
        return i;
      }
      _ => i += 1,
    };
  }

  i
}

#[inline]
fn index_of_word_end(s: &str, start: usize) -> usize {
  let bytes = s.as_bytes();
  let mut i = start;
  let len = bytes.len();

  while i < len {
    match bytes[i] as char {
      '\t' | '\n' | '\u{12}' | '\r' | ' ' | '!' | '"' | '#' | '\'' | '(' | ')' | ':' | ';'
      | '@' | '[' | '\\' | ']' | '{' | '}' => {
        return i;
      }
      '/' => {
        if bytes[i + 1] as char == '*' {
          return i;
        } else {
          i += 1;
        }
      }
      _ => i += 1,
    };
  }
  i
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
