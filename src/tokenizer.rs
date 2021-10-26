use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::clone::Clone;

use crate::input::Input;
use crate::number_type::Number;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenSymbol {
  SingleQuote,
  DoubleQuote,
  Backslash,
  Slash,
  NewLine,
  Space,
  Feed,
  Tab,
  CR,
  OpenSquare,
  CloseSquare,
  OpenParentheses,
  CloseParentheses,
  OpenCurly,
  CloseCurly,
  Semicolon,
  Asterisk,
  Colon,
  At,
  InvalidToken,
  Other(char),
}

pub trait TokenTrait {
  fn get_type(&self) -> TokenSymbol;
  fn get_content(&self) -> String;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Span {
  symbol: TokenSymbol,
  content: String,
  pos: Option<Number>,
  next: Option<Number>,
}

impl Span {
  pub fn new_with_symbol(
    symbol: TokenSymbol,
    content: &str,
    pos: Option<Number>,
    next: Option<Number>,
  ) -> Self {
    Span {
      symbol,
      pos,
      next,
      content: content.to_string(),
    }
  }
  pub fn new(content: &str, pos: Option<Number>, next: Option<Number>) -> Self {
    Span {
      symbol: if content.len() == 1 {
        content.chars().next().unwrap().into()
      } else {
        Default::default()
      },
      pos,
      next,
      content: content.to_string(),
    }
  }
}

impl TokenTrait for Span {
  fn get_type(&self) -> TokenSymbol {
    self.symbol.clone()
  }
  fn get_content(&self) -> String {
    self.content.clone()
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpanMalformed {
  pub symbol: TokenSymbol,
  pub content: String,
  pub pos: Number,
}

impl TokenTrait for SpanMalformed {
  fn get_type(&self) -> TokenSymbol {
    self.symbol.clone()
  }
  fn get_content(&self) -> String {
    self.content.clone()
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpanControl {
  pub symbol: TokenSymbol,
  pub content: String,
  pub pos: Number,
}

impl SpanControl {
  pub fn new(symbol: TokenSymbol, pos: Number) -> Self {
    SpanControl {
      symbol: symbol.clone(),
      pos,
      content: char::from(symbol).to_string(),
    }
  }
}

impl TokenTrait for SpanControl {
  fn get_type(&self) -> TokenSymbol {
    self.symbol.clone()
  }
  fn get_content(&self) -> String {
    self.content.clone()
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
  Space(Span),
  Brackets(Span),
  String(Span),
  AtWord(Span),
  Word(Span),
  Comment(Span),
  LeftParent(SpanMalformed),
  Control(SpanControl),
}

impl TokenTrait for Token {
  fn get_type(&self) -> TokenSymbol {
    match self {
      Token::Space(it) => it.get_type(),
      Token::Brackets(it) => it.get_type(),
      Token::String(it) => it.get_type(),
      Token::AtWord(it) => it.get_type(),
      Token::Word(it) => it.get_type(),
      Token::Comment(it) => it.get_type(),
      Token::LeftParent(it) => it.get_type(),
      Token::Control(it) => it.get_type(),
    }
  }
  fn get_content(&self) -> String {
    match self {
      Token::Space(it) => it.get_content(),
      Token::Brackets(it) => it.get_content(),
      Token::String(it) => it.get_content(),
      Token::AtWord(it) => it.get_content(),
      Token::Word(it) => it.get_content(),
      Token::Comment(it) => it.get_content(),
      Token::LeftParent(it) => it.get_content(),
      Token::Control(it) => it.get_content(),
    }
  }
}

impl Default for TokenSymbol {
  fn default() -> Self {
    TokenSymbol::InvalidToken
  }
}

impl ToString for TokenSymbol {
  fn to_string(&self) -> String {
    char::from(self).to_string()
  }
}

impl From<&TokenSymbol> for char {
  fn from(input: &TokenSymbol) -> Self {
    match input {
      TokenSymbol::SingleQuote => SINGLE_QUOTE,
      TokenSymbol::DoubleQuote => DOUBLE_QUOTE,
      TokenSymbol::Backslash => BACKSLASH,
      TokenSymbol::Slash => SLASH,
      TokenSymbol::NewLine => NEWLINE,
      TokenSymbol::Space => SPACE,
      TokenSymbol::Feed => FEED,
      TokenSymbol::Tab => TAB,
      TokenSymbol::CR => CR,
      TokenSymbol::OpenSquare => OPEN_SQUARE,
      TokenSymbol::CloseSquare => CLOSE_SQUARE,
      TokenSymbol::OpenParentheses => OPEN_PARENTHESES,
      TokenSymbol::CloseParentheses => CLOSE_PARENTHESES,
      TokenSymbol::OpenCurly => OPEN_CURLY,
      TokenSymbol::CloseCurly => CLOSE_CURLY,
      TokenSymbol::Semicolon => SEMICOLON,
      TokenSymbol::Asterisk => ASTERISK,
      TokenSymbol::Colon => COLON,
      TokenSymbol::At => AT,
      TokenSymbol::InvalidToken => '\0',
      TokenSymbol::Other(ch) => *ch,
    }
  }
}

impl From<TokenSymbol> for char {
  fn from(input: TokenSymbol) -> Self {
    char::from(&input)
  }
}

impl From<char> for TokenSymbol {
  fn from(input: char) -> Self {
    match input {
      SINGLE_QUOTE => TokenSymbol::SingleQuote,
      DOUBLE_QUOTE => TokenSymbol::DoubleQuote,
      BACKSLASH => TokenSymbol::Backslash,
      SLASH => TokenSymbol::Slash,
      NEWLINE => TokenSymbol::NewLine,
      SPACE => TokenSymbol::Space,
      FEED => TokenSymbol::Feed,
      TAB => TokenSymbol::Tab,
      CR => TokenSymbol::CR,
      OPEN_SQUARE => TokenSymbol::OpenSquare,
      CLOSE_SQUARE => TokenSymbol::CloseSquare,
      OPEN_PARENTHESES => TokenSymbol::OpenParentheses,
      CLOSE_PARENTHESES => TokenSymbol::CloseParentheses,
      OPEN_CURLY => TokenSymbol::OpenCurly,
      CLOSE_CURLY => TokenSymbol::CloseCurly,
      SEMICOLON => TokenSymbol::Semicolon,
      ASTERISK => TokenSymbol::Asterisk,
      COLON => TokenSymbol::Colon,
      AT => TokenSymbol::At,
      '\0' => TokenSymbol::InvalidToken,
      other => TokenSymbol::Other(other),
    }
  }
}

lazy_static! {
  static ref RE_AT_END: Regex = Regex::new(r##"[\t\n\u{12}\r "#'()/;\[\\\]{}]"##).unwrap();
  static ref RE_WORD_END: Regex =
    Regex::new(r##"[\t\n\u{12}\r !"#'():;@\[\\\]{}]|/(?=\*)"##).unwrap();
  static ref RE_BAD_BRACKET: Regex = Regex::new(r#".[\n"'(\/\\]"#).unwrap();
  static ref RE_HEX_ESCAPE: Regex = Regex::new(r"[\da-f]").unwrap();
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
  css: &'a str,
  ignore: bool,
  current_token: Option<Token>,
  length: Number,
  pos: Number,
  buffer: Vec<Token>,
  returned: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
  pub fn new(input: &'a Input, ignore_errors: bool) -> Tokenizer {
    let length = input.css.chars().count() as Number;
    Tokenizer {
      css: &input.css,
      ignore: ignore_errors,
      current_token: None,
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

  pub fn position(&self) -> Number {
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
    if !self.returned.is_empty() {
      return self.returned.pop();
    }

    if self.end_of_file() {
      return None;
    }

    let mut code: TokenSymbol = char_code_at(self.css, self.pos).into();

    match code {
      TokenSymbol::NewLine
      | TokenSymbol::Space
      | TokenSymbol::Tab
      | TokenSymbol::CR
      | TokenSymbol::Feed => {
        let mut next = self.pos;
        loop {
          next += 1;
          code = char_code_at(self.css, next).into();
          if !(code == TokenSymbol::Space
            || code == TokenSymbol::NewLine
            || code == TokenSymbol::Tab
            || code == TokenSymbol::Feed)
          {
            break;
          }
        }

        self.current_token = Some(Token::Space(Span::new(
          &self.css[(self.pos as usize)..(next as usize)],
          None,
          None,
        )));

        self.pos = next - 1;
      }

      ch
      @
      (TokenSymbol::OpenSquare
      | TokenSymbol::CloseSquare
      | TokenSymbol::OpenCurly
      | TokenSymbol::CloseCurly
      | TokenSymbol::Colon
      | TokenSymbol::Semicolon
      | TokenSymbol::CloseParentheses) => {
        self.current_token = Some(Token::Control(SpanControl {
          symbol: ch.clone(),
          content: ch.to_string(),
          pos: self.pos,
        }));
      }

      TokenSymbol::OpenParentheses => {
        let prev = self
          .buffer
          .pop()
          .map(|it| it.get_content())
          .unwrap_or_else(|| "".to_string());
        let n = char_code_at(self.css, self.pos + 1);
        if prev == "url"
          && n != TokenSymbol::SingleQuote
          && n != TokenSymbol::DoubleQuote
          && n != TokenSymbol::Space
          && n != TokenSymbol::NewLine
          && n != TokenSymbol::Tab
          && n != TokenSymbol::Feed
          && n != TokenSymbol::CR
        {
          let mut next = self.pos;
          loop {
            let mut escaped = false;
            match index_of(self.css, ")", next + 1) {
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
            while char_code_at(self.css, escape_pos - 1) == TokenSymbol::Backslash {
              escape_pos -= 1;
              escaped = !escaped;
            }

            if !escaped {
              break;
            }
          }

          self.current_token = Some(Token::Brackets(Span::new(
            sub_string(self.css, self.pos, next + 1),
            Some(self.pos),
            Some(next),
          )));

          self.pos = next;
        } else {
          match index_of_char(self.css, ')', self.pos + 1) {
            Some(i) => {
              let content = &self.css[self.pos as usize..i as usize + 1];

              if RE_BAD_BRACKET.is_match(content).unwrap_or(false) {
                self.current_token = Some(Token::LeftParent(SpanMalformed {
                  symbol: '('.into(),
                  content: "(".to_string(),
                  pos: self.pos,
                }));
              } else {
                self.current_token =
                  Some(Token::Brackets(Span::new(content, Some(self.pos), Some(i))));
                self.pos = i;
              }
            }
            None => {
              // self.current_token = Token("(", "(".to_string(), Some(self.pos), None);
              self.current_token = Some(Token::LeftParent(SpanMalformed {
                symbol: '('.into(),
                content: "(".to_string(),
                pos: self.pos,
              }));
            }
          };
        }
      }
      TokenSymbol::SingleQuote | TokenSymbol::DoubleQuote => {
        let quote = if code == SINGLE_QUOTE.into() {
          '\''
        } else {
          '"'
        };
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
          while char_code_at(self.css, escape_pos - 1) == TokenSymbol::Backslash {
            escape_pos -= 1;
            escaped = !escaped;
          }

          if !escaped {
            break;
          }
        }

        self.current_token = Some(Token::String(Span::new(
          sub_string(self.css, self.pos, next + 1),
          Some(self.pos),
          Some(next),
        )));
        self.pos = next;
      }
      TokenSymbol::At => {
        let next = match RE_AT_END.find(&self.css[self.pos as usize + 1..]).unwrap() {
          Some(mat) => self.pos + 1 + mat.end() as Number - 2,
          None => self.length - 1,
        };
        self.current_token = Some(Token::AtWord(Span::new(
          sub_string(self.css, self.pos, next + 1),
          Some(self.pos),
          Some(next),
        )));
        self.pos = next;
      }
      TokenSymbol::Backslash => {
        let mut next = self.pos;
        let mut escape = true;
        while char_code_at(self.css, next + 1) == TokenSymbol::Backslash {
          next += 1;
          escape = !escape;
        }
        code = char_code_at(self.css, next + 1).into();
        if escape
          && code != TokenSymbol::Slash
          && code != TokenSymbol::Space
          && code != TokenSymbol::NewLine
          && code != TokenSymbol::Tab
          && code != TokenSymbol::CR
          && code != TokenSymbol::Feed
        {
          next += 1;
          if RE_HEX_ESCAPE
            .is_match(sub_string(self.css, next, next + 1))
            .unwrap_or(false)
          {
            while RE_HEX_ESCAPE
              .is_match(sub_string(self.css, next + 1, next + 2))
              .unwrap_or(false)
            {
              next += 1;
            }
            if char_code_at(self.css, next + 1) == TokenSymbol::Space {
              next += 1;
            }
          }
        }

        self.current_token = Some(Token::Word(Span::new(
          sub_string(self.css, self.pos, next + 1),
          Some(self.pos),
          Some(next),
        )));
        self.pos = next;
      }
      _ => {
        self.pos = if code == TokenSymbol::Slash
          && char_code_at(self.css, self.pos + 1) == TokenSymbol::Asterisk
        {
          let next = match index_of(self.css, "*/", self.pos + 2) {
            Some(i) => i + 1,
            None => {
              if !self.ignore && !ignore_unclosed {
                self.unclosed("comment");
              }
              self.length
            }
          };

          self.current_token = Some(Token::Comment(Span::new(
            sub_string(self.css, self.pos, next + 1),
            Some(self.pos),
            Some(next),
          )));

          next
        } else {
          let next = match RE_WORD_END
            .find(&self.css[self.pos as usize + 1..])
            .unwrap()
          {
            Some(mat) => self.pos + mat.end() as i32 - 1,
            None => self.length - 1,
          };
          self.current_token = Some(Token::Word(Span::new(
            sub_string(self.css, self.pos, next + 1),
            Some(self.pos),
            Some(next),
          )));

          self.push(self.current_token.as_ref().unwrap().clone());
          next
        }
      }
    }

    self.pos += 1;
    self.current_token.clone()
  }
}

#[inline]
fn index_of(value: &str, search_value: &str, from_index: Number) -> Option<Number> {
  assert!(from_index >= 0);
  let (_, last) = value.split_at(from_index as usize);
  last.find(search_value).map(|v| v as Number + from_index)
}

#[inline]
fn index_of_char(value: &str, search_value: char, from_index: Number) -> Option<Number> {
  let (_, last) = value.split_at(from_index as usize);
  last.find(search_value).map(|v| v as Number + from_index)
}

#[inline]
fn sub_string(s: &str, start: Number, end: Number) -> &str {
  if end + 1 > s.len() as Number {
    &s[start as usize..]
  } else {
    &s[start as usize..end as usize]
  }
}

#[inline]
fn char_code_at(s: &str, n: Number) -> TokenSymbol {
  if n >= s.len() as Number {
    '\0'.into()
  } else {
    (s.as_bytes()[n as usize] as char).into()
  }
  // s.chars().nth(n).unwrap_or('\0')
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_char_code_at() {
    let s = "0123456789abc";
    assert_eq!(char_code_at(s, 0), '0'.into());
    assert_eq!(char_code_at(s, 1), '1'.into());
    assert_eq!(char_code_at(s, 100), '\0'.into());
  }
}
