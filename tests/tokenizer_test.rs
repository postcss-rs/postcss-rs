use postcss::input::Input;
use postcss::tokenizer::*;

fn tokenize(css: &str, ignore_errors: bool) -> Vec<Token> {
  let input = Input::new(css.to_string(), None);
  let mut processor = Tokenizer::new(&input, ignore_errors);
  let mut tokens = vec![];
  while !processor.end_of_file() {
    tokens.push(processor.next_token(false).unwrap())
  }
  return tokens;
}

fn run(css: &str, tokens: Vec<Token>) {
  assert_eq!(tokenize(css, false), tokens);
}

fn run_ignore_errors(css: &str, tokens: Vec<Token>) {
  assert_eq!(tokenize(css, true), tokens);
}

#[test]
fn tokenizes_empty_file() {
  run("", vec![]);
}

#[test]
fn tokenizes_space() {
  run(
    "\r\n \u{12}\t",
    vec![Token::Space(Span::new_with_symbol(
      Default::default(),
      "\r\n \u{12}\t",
      None,
      None,
    ))],
  );
}

#[test]
fn tokenizes_word() {
  run("ab", vec![Token::Word(Span::new("ab", Some(0), Some(1)))]);
}

#[test]
fn splits_word_by_exclamation_mark() {
  run(
    "aa!bb",
    vec![
      Token::Word(Span::new("aa", Some(0), Some(1))),
      Token::Word(Span::new("!bb", Some(2), Some(4))),
    ],
  );
}

#[test]
fn changes_lines_in_spaces() {
  run(
    "a \n b",
    vec![
      Token::Word(Span::new("a", Some(0), Some(0))),
      Token::Space(Span::new(" \n ", None, None)),
      Token::Word(Span::new("b", Some(4), Some(4))),
    ],
  );
}

#[test]
fn tokenizes_control_chars() {
  run(
    "{:;}",
    vec![
      Token::Control(SpanControl {
        symbol: TokenSymbol::OpenCurly,
        content: TokenSymbol::OpenCurly.to_string(),
        pos: 0,
      }),
      Token::Control(SpanControl {
        symbol: TokenSymbol::Colon,
        content: TokenSymbol::Colon.to_string(),
        pos: 1,
      }),
      Token::Control(SpanControl {
        symbol: TokenSymbol::Semicolon,
        content: TokenSymbol::Semicolon.to_string(),
        pos: 2,
      }),
      Token::Control(SpanControl {
        symbol: TokenSymbol::CloseCurly,
        content: TokenSymbol::CloseCurly.to_string(),
        pos: 3,
      }),
    ],
  );
}

#[test]
fn escapes_control_symbols() {
  run(
    "\\(\\{\\\"\\@\\\\\"\"",
    vec![
      Token::Word(Span::new("\\(", Some(0), Some(1))),
      Token::Word(Span::new("\\{", Some(2), Some(3))),
      Token::Word(Span::new("\\\"", Some(4), Some(5))),
      Token::Word(Span::new("\\@", Some(6), Some(7))),
      Token::Word(Span::new("\\\\", Some(8), Some(9))),
      Token::String(Span::new("\"\"", Some(10), Some(11))),
    ],
  );
}

#[test]
fn escapes_backslash() {
  run(
    "\\\\\\\\{",
    vec![
      Token::Word(Span::new("\\\\\\\\", Some(0), Some(3))),
      Token::Control(SpanControl::new(TokenSymbol::OpenCurly, 4)),
    ],
  );
}

#[test]
fn tokenizes_simple_brackets() {
  run(
    "(ab)",
    vec![Token::Brackets(Span::new("(ab)", Some(0), Some(3)))],
  );
}

#[test]
fn tokenizes_square_brackets() {
  run(
    "a[bc]",
    vec![
      Token::Word(Span::new("a", Some(0), Some(0))),
      Token::Control(SpanControl::new(TokenSymbol::OpenSquare, 1)),
      Token::Word(Span::new("bc", Some(2), Some(3))),
      Token::Control(SpanControl::new(TokenSymbol::CloseSquare, 4)),
    ],
  );
}

#[test]
fn tokenizes_complicated_brackets() {
  run(
    "(())(\"\")(/**/)(\\\\)(\n)(",
    vec![
      Token::LeftParent(SpanMalformed {
        symbol: '('.into(),
        content: "(".to_string(),
        pos: 0,
      }),
      Token::Brackets(Span::new("()", Some(1), Some(2))),
      Token::Control(SpanControl::new(TokenSymbol::CloseParentheses, 3)),
      Token::LeftParent(SpanMalformed {
        symbol: '('.into(),
        content: "(".to_string(),
        pos: 4,
      }),
      Token::String(Span::new("\"\"", Some(5), Some(6))),
      Token::Control(SpanControl::new(TokenSymbol::CloseParentheses, 7)),
      Token::LeftParent(SpanMalformed {
        symbol: '('.into(),
        content: "(".to_string(),
        pos: 8,
      }),
      Token::Comment(Span::new("/**/", Some(9), Some(12))),
      Token::Control(SpanControl::new(TokenSymbol::CloseParentheses, 13)),
      Token::LeftParent(SpanMalformed {
        symbol: '('.into(),
        content: "(".to_string(),
        pos: 14,
      }),
      Token::Word(Span::new("\\\\", Some(15), Some(16))),
      Token::Control(SpanControl::new(TokenSymbol::CloseParentheses, 17)),
      Token::LeftParent(SpanMalformed {
        symbol: '('.into(),
        content: "(".to_string(),
        pos: 18,
      }),
      Token::Space(Span::new("\n", None, None)),
      Token::Control(SpanControl::new(TokenSymbol::CloseParentheses, 20)),
      Token::LeftParent(SpanMalformed {
        symbol: '('.into(),
        content: "(".to_string(),
        pos: 21,
      }),
    ],
  );
}

#[test]
fn tokenizes_string() {
  run(
    "'\"'\"\\\"\"",
    vec![
      Token::String(Span::new("'\"'", Some(0), Some(2))),
      Token::String(Span::new("\"\\\"\"", Some(3), Some(6))),
    ],
  );
}

#[test]
fn tokenizes_escaped_string() {
  run(
    "\"\\\\\"",
    vec![Token::String(Span::new("\"\\\\\"", Some(0), Some(3)))],
  );
}

#[test]
fn changes_lines_in_strings() {
  run(
    "\"\n\n\"\"\n\n\"",
    vec![
      Token::String(Span::new("\"\n\n\"", Some(0), Some(3))),
      Token::String(Span::new("\"\n\n\"", Some(4), Some(7))),
    ],
  );
}

#[test]
fn tokenizes_at_word() {
  run(
    "@word ",
    vec![
      Token::AtWord(Span::new("@word", Some(0), Some(4))),
      Token::Space(Span::new(" ", None, None)),
    ],
  );
}

#[test]
fn tokenizes_at_word_end() {
  run(
    "@one{@two()@three\"\"@four;",
    vec![
      Token::AtWord(Span::new("@one", Some(0), Some(3))),
      Token::Control(SpanControl::new(TokenSymbol::OpenCurly, 4)),
      Token::AtWord(Span::new("@two", Some(5), Some(8))),
      Token::Brackets(Span::new("()", Some(9), Some(10))),
      Token::AtWord(Span::new("@three", Some(11), Some(16))),
      Token::String(Span::new("\"\"", Some(17), Some(18))),
      Token::AtWord(Span::new("@four", Some(19), Some(23))),
      Token::Control(SpanControl::new(TokenSymbol::Semicolon, 24)),
    ],
  );
}

#[test]
fn tokenizes_urls() {
  run(
    "url(/*\\))",
    vec![
      Token::Word(Span::new("url", Some(0), Some(2))),
      Token::Brackets(Span::new("(/*\\))", Some(3), Some(8))),
    ],
  );
}

#[test]
fn tokenizes_quoted_urls() {
  run(
    "url(\")\")",
    vec![
      Token::Word(Span::new("url", Some(0), Some(2))),
      Token::LeftParent(SpanMalformed {
        symbol: '('.into(),
        content: "(".to_string(),
        pos: 3,
      }),
      Token::String(Span::new("\")\"", Some(4), Some(6))),
      Token::Control(SpanControl::new(TokenSymbol::CloseParentheses, 7)),
    ],
  );
}

#[test]
fn tokenizes_at_symbol() {
  run("@", vec![Token::AtWord(Span::new("@", Some(0), Some(0)))]);
}

#[test]
fn tokenizes_comment() {
  run(
    "/* a\nb */",
    vec![Token::Comment(Span::new("/* a\nb */", Some(0), Some(8)))],
  );
}

#[test]
fn changes_lines_in_comments() {
  run(
    "a/* \n */b",
    vec![
      Token::Word(Span::new("a", Some(0), Some(0))),
      Token::Comment(Span::new("/* \n */", Some(1), Some(7))),
      Token::Word(Span::new("b", Some(8), Some(8))),
    ],
  );
}

#[test]
fn supports_line_feed() {
  run(
    "a\u{12}b",
    vec![
      Token::Word(Span::new("a", Some(0), Some(0))),
      Token::Space(Span::new("\u{12}", None, None)),
      Token::Word(Span::new("b", Some(2), Some(2))),
    ],
  );
}

#[test]
fn supports_carriage_return() {
  run(
    "a\rb\r\nc",
    vec![
      Token::Word(Span::new("a", Some(0), Some(0))),
      Token::Space(Span::new("\r", None, None)),
      Token::Word(Span::new("b", Some(2), Some(2))),
      Token::Space(Span::new("\r\n", None, None)),
      Token::Word(Span::new("c", Some(5), Some(5))),
    ],
  );
}

#[test]
fn tokenizes_css() {
  run(
    "a {\n  content: \"a\";\n  width: calc(1px;)\n  }\n/* small screen */\n@media screen {}",
    vec![
      Token::Word(Span::new("a", Some(0), Some(0))),
      Token::Space(Span::new(" ", None, None)),
      Token::Control(SpanControl::new(TokenSymbol::OpenCurly, 2)),
      Token::Space(Span::new("\n  ", None, None)),
      Token::Word(Span::new("content", Some(6), Some(12))),
      Token::Control(SpanControl::new(TokenSymbol::Colon, 13)),
      Token::Space(Span::new(" ", None, None)),
      Token::String(Span::new("\"a\"", Some(15), Some(17))),
      Token::Control(SpanControl::new(TokenSymbol::Semicolon, 18)),
      Token::Space(Span::new("\n  ", None, None)),
      Token::Word(Span::new("width", Some(22), Some(26))),
      Token::Control(SpanControl::new(TokenSymbol::Colon, 27)),
      Token::Space(Span::new(" ", None, None)),
      Token::Word(Span::new("calc", Some(29), Some(32))),
      Token::Brackets(Span::new("(1px;)", Some(33), Some(38))),
      Token::Space(Span::new("\n  ", None, None)),
      Token::Control(SpanControl::new(TokenSymbol::CloseCurly, 42)),
      Token::Space(Span::new("\n", None, None)),
      Token::Comment(Span::new("/* small screen */", Some(44), Some(61))),
      Token::Space(Span::new("\n", None, None)),
      Token::AtWord(Span::new("@media", Some(63), Some(68))),
      Token::Space(Span::new(" ", None, None)),
      Token::Word(Span::new("screen", Some(70), Some(75))),
      Token::Space(Span::new(" ", None, None)),
      Token::Control(SpanControl::new(TokenSymbol::OpenCurly, 77)),
      Token::Control(SpanControl::new(TokenSymbol::CloseCurly, 78)),
    ],
  );
}

#[test]
#[should_panic(expected = "Unclosed string 1")]
fn throws_error_on_unclosed_string() {
  tokenize(" \"", false);
}

#[test]
#[should_panic(expected = "Unclosed comment 1")]
fn throws_error_on_unclosed_comment() {
  tokenize(" /*", false);
}

#[test]
#[should_panic(expected = "Unclosed bracket 3")]
fn throws_error_on_unclosed_url() {
  tokenize("url(", false);
}

#[test]
fn ignores_unclosing_string_on_request() {
  run_ignore_errors(
    " \"",
    vec![
      Token::Space(Span::new(" ", None, None)),
      Token::String(Span::new("\"", Some(1), Some(2))),
    ],
  );
}

#[test]
fn ignores_unclosing_comment_on_request() {
  run_ignore_errors(
    " /*",
    vec![
      Token::Space(Span::new(" ", None, None)),
      Token::Comment(Span::new("/*", Some(1), Some(3))),
    ],
  );
}

#[test]
fn ignores_unclosing_function_on_request() {
  run_ignore_errors(
    "url(",
    vec![
      Token::Word(Span::new("url", Some(0), Some(2))),
      Token::Brackets(Span::new("(", Some(3), Some(3))),
    ],
  );
}

#[test]
fn tokenizes_hexadecimal_escape() {
  run(
    "\\0a \\09 \\z ",
    vec![
      Token::Word(Span::new("\\0a ", Some(0), Some(3))),
      Token::Word(Span::new("\\09 ", Some(4), Some(7))),
      Token::Word(Span::new("\\z", Some(8), Some(9))),
      Token::Space(Span::new(" ", None, None)),
    ],
  );
}

#[test]
fn ignore_unclosed_per_token_request() {
  fn tokn(css: &str) -> Vec<Token> {
    let input = Input::new(css.to_string(), None);
    let mut processor = Tokenizer::new(&input, false);
    let mut tokens = vec![];
    while !processor.end_of_file() {
      tokens.push(processor.next_token(true).unwrap())
    }
    return tokens;
  }

  let tokens = tokn("How's it going (");
  let expected = vec![
    Token::Word(Span::new("How", Some(0), Some(2))),
    Token::String(Span::new("'s", Some(3), Some(4))),
    Token::Space(Span::new(" ", None, None)),
    Token::Word(Span::new("it", Some(6), Some(7))),
    Token::Space(Span::new(" ", None, None)),
    Token::Word(Span::new("going", Some(9), Some(13))),
    Token::Space(Span::new(" ", None, None)),
    Token::LeftParent(SpanMalformed {
      symbol: '('.into(),
      content: "(".to_string(),
      pos: 15,
    }),
  ];
  assert_eq!(tokens, expected);
}

#[test]
fn provides_correct_position() {
  let css = "Three tokens";
  let input = Input::new(css.to_string(), None);
  let mut processor = Tokenizer::new(&input, false);
  assert_eq!(processor.position(), 0);
  processor.next_token(false);
  assert_eq!(processor.position(), 5);
  processor.next_token(false);
  assert_eq!(processor.position(), 6);
  processor.next_token(false);
  assert_eq!(processor.position(), 12);
  // processor.next_token(false);
  // assert_eq!(processor.position(), 12);
}
