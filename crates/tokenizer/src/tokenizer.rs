use crate::ref_ring::RefRing;
use memchr::memchr;
use memchr::memmem::Finder;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::clone::Clone;
use std::cmp::PartialEq;
use std::cmp::{min, Eq};
use std::collections::HashMap;

const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';
const BACKSLASH: char = '\\';
const SLASH: char = '/';
const NEWLINE: char = '\n';
const SPACE: char = ' ';
const FEED: char = '\u{c}'; // \f
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

const MAX_BUFFER: usize = 102400;

static FINDER_END_OF_COMMENT: Lazy<Finder<'static>> = Lazy::new(|| Finder::new("*/"));

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenType {
  OpenParentheses,
  CloseParentheses,
  Space,
  Word,
  String,
  OpenSquare,
  CloseSquare,
  OpenCurly,
  CloseCurly,
  Semicolon,
  Colon,
  Comment,
  AtWord,
  Brackets,
  Unknown,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token<'a>(pub TokenType, pub &'a str, pub usize, pub usize);

impl<'a> Token<'a> {
  pub fn new(kind: TokenType, content: &'a str, pos: usize, next: usize) -> Token {
    Token(kind, content, pos, next)
  }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
  pub css: &'a str,
  ignore: bool,
  length: usize,
  pos: RefCell<usize>,
  buffer: RefCell<RefRing<'a>>,
  returned: RefCell<Vec<Token<'a>>>,
  rope: Option<ropey::Rope>,
  from_offset_cache: Option<HashMap<usize, usize>>,
}

impl<'a> Tokenizer<'a> {
  pub fn new(source_code: &'a str, ignore_errors: bool) -> Tokenizer<'a> {
    let length = source_code.len();
    Tokenizer {
      css: source_code,
      ignore: ignore_errors,
      length,
      pos: RefCell::new(0),
      buffer: RefCell::new(Default::default()),
      returned: RefCell::new(Vec::with_capacity(min(MAX_BUFFER, length / 8))),
      rope: None,
      from_offset_cache: None,
    }
  }

  #[inline]
  fn push(&self, t: &'a str) {
    self.buffer.borrow_mut().push(t);
  }

  #[inline]
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

        current_token = Token(
          TokenType::Space,
          self.css[self.position()..next].into(),
          self.position(),
          next,
        );

        self.pos.replace(next);
      }
      OPEN_SQUARE | CLOSE_SQUARE | OPEN_CURLY | CLOSE_CURLY | COLON | SEMICOLON
      | CLOSE_PARENTHESES => {
        let start = self.position();
        current_token = Token(get_token_type(code), get_str(code), start, start + 1);
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
          let start_offset = self.position();
          current_token = Token(
            TokenType::Brackets,
            sub_str(self.css, start_offset, next + 1),
            start_offset,
            next + 1,
          );

          self.pos.replace(next + 1);
        } else {
          match index_of_byte(self.css, b')', self.position() + 1) {
            Some(i) => {
              let content = &self.css[self.position()..i + 1];

              let start_offset = self.position();
              if is_bad_bracket(content) {
                current_token = Token(
                  TokenType::OpenParentheses,
                  "(",
                  start_offset,
                  start_offset + 1,
                );
              } else {
                current_token = Token(TokenType::Brackets, content, start_offset, i + 1);
                self.pos.replace(i);
              }
            }
            None => {
              let start_offset = self.position();
              current_token = Token(
                TokenType::OpenParentheses,
                "(",
                start_offset,
                start_offset + 1,
              );
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
          TokenType::String,
          sub_str(self.css, self.position(), next + 1),
          self.position(),
          next + 1,
        );
        self.pos.replace(next + 1);
      }
      AT => {
        let next = index_of_at_end(self.css, self.position() + 1) - 1;
        current_token = Token(
          TokenType::AtWord,
          sub_str(self.css, self.position(), next + 1),
          self.position(),
          next + 1,
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
          TokenType::Word,
          sub_str(self.css, self.position(), next + 1),
          self.position(),
          next + 1,
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
                self.length - 1
              }
            };

            current_token = Token(
              TokenType::Comment,
              sub_str(self.css, self.position(), next + 1),
              self.position(),
              next + 1,
            );
            next
          } else {
            let next = index_of_word_end(self.css, self.position() + 1) - 1;
            let content = sub_str(self.css, self.position(), next + 1);
            current_token = Token::new(TokenType::Word, content, self.position(), next + 1);
            self.push(content);
            next
          },
        );
        self.pos_plus_one();
      }
    }

    current_token
  }

  /// return (line, column), use rope for simplicity
  pub fn from_offset(&mut self, offset: usize) -> (usize, usize) {
    let rope = if let Some(ref rope) = self.rope {
      rope
    } else {
      self.rope = Some(ropey::Rope::from_str(self.css));
      &self.rope.as_ref().unwrap()
    };
    let column = rope.byte_to_char(offset);
    let line = rope.byte_to_line(offset);
    (line, column)
  }

  pub fn from_offset2(&mut self, offset: usize) -> (usize, usize) {
    let mut last_line: usize = 0;
    let mut line_to_index: &mut HashMap<usize, usize>;
    if let Some(cache) = &mut self.from_offset_cache {
      line_to_index = cache;
    } else {
      let lines = self.css.split('\n').collect::<Vec<&str>>();
      self.from_offset_cache = Some(HashMap::with_capacity(lines.len()));
      line_to_index = self.from_offset_cache.as_mut().unwrap();
      let mut prev_index = 0;
      for i in 0..lines.len() {
        line_to_index.insert(i, prev_index);
        prev_index += lines[i].len() + 1;
      }
    }
    last_line = line_to_index[&(line_to_index.len() - 1)];

    let mut min = 0;
    if offset >= last_line {
      min = line_to_index.len() - 1;
    } else {
      let mut max = line_to_index.len() - 2;
      let mut mid = 0usize;
      while min < max {
        mid = min + ((max - min) >> 1);
        if offset < line_to_index[&mid] {
          max = mid - 1;
        } else if offset >= line_to_index[&(mid + 1)] {
          min = mid + 1;
        } else {
          min = mid;
          break;
        }
      }
    }

    (min + 1, offset - line_to_index[&min] + 1)
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
fn sub_str(s: &str, start: usize, end: usize) -> &str {
  if end + 1 > s.len() {
    // Safety: NEVER out-of-bounds
    unsafe { s.get_unchecked(start..) }
  } else {
    // Safety: NEVER out-of-bounds
    unsafe { s.get_unchecked(start..end) }
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
  #[allow(clippy::needless_range_loop)]
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
      '\t' | '\n' | '\u{c}' | '\r' | ' ' | '"' | '#' | '\'' | '(' | ')' | '/' | ';' | '['
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
      '\t' | '\n' | '\u{c}' | '\r' | ' ' | '!' | '"' | '#' | '\'' | '(' | ')' | ':' | ';' | '@'
      | '[' | '\\' | ']' | '{' | '}' => {
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

/// SAFETY: YOU SHOULD NEVER CALL THIS FUNCTION WITH THE PARAM OTHER THAN THESE BELOW.
const fn get_str(ch: char) -> &'static str {
  match ch {
    OPEN_SQUARE => "[",
    CLOSE_SQUARE => "]",
    OPEN_CURLY => "{",
    CLOSE_CURLY => "}",
    COLON => ":",
    SEMICOLON => ";",
    CLOSE_PARENTHESES => ")",
    _ => "",
  }
}

/// SAFETY: YOU SHOULD NEVER CALL THIS FUNCTION WITH THE PARAM OTHER THAN THESE BELOW.
const fn get_token_type(ch: char) -> TokenType {
  match ch {
    OPEN_SQUARE => TokenType::OpenSquare,
    CLOSE_SQUARE => TokenType::CloseSquare,
    OPEN_CURLY => TokenType::OpenCurly,
    CLOSE_CURLY => TokenType::CloseCurly,
    COLON => TokenType::Colon,
    SEMICOLON => TokenType::Semicolon,
    CLOSE_PARENTHESES => TokenType::CloseParentheses,
    _ => TokenType::Unknown,
  }
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

  #[test]
  fn test_sub_str() {
    let s = "0123456789abc";
    assert_eq!(sub_str(s, 0, 0), "");
    assert_eq!(sub_str(s, 1, 3), "12");
    assert_eq!(sub_str(s, 10, 13), "abc");
    assert_eq!(sub_str(s, 10, 100), "abc");
  }
}
