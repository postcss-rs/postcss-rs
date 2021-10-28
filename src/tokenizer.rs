use crate::input::Input;
use lazy_static::lazy_static;
use memchr::memchr;
use memchr::memmem::Finder;
use smol_str::SmolStr;
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
pub struct Token(
  pub SmolStr,
  pub SmolStr,
  pub Option<usize>,
  pub Option<usize>,
);

impl Token {
  pub fn new(kind: &'static str, content: &str, pos: Option<usize>, next: Option<usize>) -> Token {
    Token(kind.into(), content.into(), pos, next)
  }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
  css: &'a str,
  ignore: bool,
  length: usize,
  pos: usize,
  buffer: Vec<&'a str>,
  returned: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
  pub fn new(input: &'a Input, ignore_errors: bool) -> Tokenizer {
    let length = input.css.len();
    Tokenizer {
      css: &input.css,
      ignore: ignore_errors,
      length,
      pos: 0,
      // buffer: Vec::with_capacity(length / 13),
      buffer: vec![],
      returned: vec![],
    }
  }

  #[inline]
  fn push(&mut self, t: &'a str) {
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

  pub fn next_token(&mut self, ignore_unclosed: bool) -> Token {
    if !self.returned.is_empty() {
      return self.returned.pop().unwrap();
    }

    let mut code = char_code_at(self.css, self.pos);

    let current_token: Token;

    match code {
      NEWLINE | SPACE | TAB | CR | FEED => {
        let mut next = self.pos;
        loop {
          next += 1;
          code = char_code_at(self.css, next);
          if !(code == SPACE || code == NEWLINE || code == TAB || code == FEED) {
            break;
          }
        }

        current_token = Token("space".into(), self.css[self.pos..next].into(), None, None);

        self.pos = next;
      }
      OPEN_SQUARE => {
        current_token = Token("[".into(), "[".into(), Some(self.pos), None);
        self.pos += 1;
      }
      CLOSE_SQUARE => {
        current_token = Token("]".into(), "]".into(), Some(self.pos), None);
        self.pos += 1;
      }
      OPEN_CURLY => {
        current_token = Token("{".into(), "{".into(), Some(self.pos), None);
        self.pos += 1;
      }
      CLOSE_CURLY => {
        current_token = Token("}".into(), "}".into(), Some(self.pos), None);
        self.pos += 1;
      }
      COLON => {
        current_token = Token(":".into(), ":".into(), Some(self.pos), None);
        self.pos += 1;
      }
      SEMICOLON => {
        current_token = Token(";".into(), ";".into(), Some(self.pos), None);
        self.pos += 1;
      }
      CLOSE_PARENTHESES => {
        current_token = Token(")".into(), ")".into(), Some(self.pos), None);
        self.pos += 1;
      }
      OPEN_PARENTHESES => {
        let prev = self.buffer.pop().unwrap_or("");
        let n = char_code_at(self.css, self.pos + 1);
        if prev == "url"
          && n != SINGLE_QUOTE
          && n != DOUBLE_QUOTE
          && n != SPACE
          && n != NEWLINE
          && n != TAB
          && n != FEED
          && n != CR
        {
          let mut next = self.pos;
          loop {
            let mut escaped = false;
            match index_of_char(self.css, ')', next + 1) {
              Some(i) => {
                next = i;
              }
              None => {
                if self.ignore || ignore_unclosed {
                  next = self.pos;
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
            "brackets".into(),
            sub_string(self.css, self.pos, next + 1).into(),
            Some(self.pos),
            Some(next),
          );

          self.pos = next + 1;
        } else {
          match index_of_char(self.css, ')', self.pos + 1) {
            Some(i) => {
              let content = &self.css[self.pos..i + 1];

              if is_bad_bracket(content) {
                current_token = Token("(".into(), "(".into(), Some(self.pos), None);
              } else {
                current_token = Token("brackets".into(), content.into(), Some(self.pos), Some(i));
                self.pos = i;
              }
            }
            None => {
              current_token = Token("(".into(), "(".into(), Some(self.pos), None);
            }
          };
          self.pos += 1;
        }
      }
      SINGLE_QUOTE | DOUBLE_QUOTE => {
        let quote = if code == SINGLE_QUOTE { '\'' } else { '"' };
        let mut next = self.pos;
        loop {
          let mut escaped = false;
          match index_of_char(self.css, quote, next + 1) {
            Some(i) => {
              next = i;
            }
            None => {
              if self.ignore || ignore_unclosed {
                next = self.pos + 1;
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
          "string".into(),
          sub_string(self.css, self.pos, next + 1).into(),
          Some(self.pos),
          Some(next),
        );
        self.pos = next + 1;
      }
      AT => {
        let next = index_of_at_end(self.css, self.pos + 1) - 1;
        current_token = Token(
          "at-word".into(),
          sub_string(self.css, self.pos, next + 1).into(),
          Some(self.pos),
          Some(next),
        );
        self.pos = next + 1;
      }
      BACKSLASH => {
        let mut next = self.pos;
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
          "word".into(),
          sub_string(self.css, self.pos, next + 1).into(),
          Some(self.pos),
          Some(next),
        );
        self.pos = next + 1;
      }
      _ => {
        self.pos = if code == SLASH && char_code_at(self.css, self.pos + 1) == ASTERISK {
          let next = match index_of_end_comment(self.css, self.pos + 2) {
            Some(i) => i + 1,
            None => {
              if !self.ignore && !ignore_unclosed {
                self.unclosed("comment");
              }
              self.length
            }
          };

          current_token = Token(
            "comment".into(),
            sub_string(self.css, self.pos, next + 1).into(),
            Some(self.pos),
            Some(next),
          );
          next
        } else {
          let next = index_of_word_end(self.css, self.pos + 1) - 1;
          let content = sub_string(self.css, self.pos, next + 1);
          current_token = Token("word".into(), content.into(), Some(self.pos), Some(next));
          self.push(content);
          next
        };
        self.pos += 1;
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
fn index_of_char(value: &str, search_value: char, from_index: usize) -> Option<usize> {
  let (_, last) = value.split_at(from_index);
  memchr(search_value as u8, last.as_bytes()).map(|v| v + from_index)
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
