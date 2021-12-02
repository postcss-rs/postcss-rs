use tokenizer::input::Input;
use tokenizer::*;
fn tokenize(css: &str, ignore_errors: bool) -> Vec<Token> {
  let input = Input::new(css, None);
  let processor = Tokenizer::new(input.css, ignore_errors);
  let mut tokens = vec![];
  while !processor.end_of_file() {
    tokens.push(processor.next_token(false))
  }
  tokens
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
  run("\r\n \u{c}\t", vec![Token::new(TokenType::Space, 0, 5)]);
}

#[test]
fn tokenizes_word() {
  run("ab", vec![Token::new(TokenType::Word, 0, 2)]);
}

#[test]
fn splits_word_by_exclamation_mark() {
  run(
    "aa!bb",
    vec![
      Token::new(TokenType::Word, 0, 2),
      Token::new(TokenType::Word, 2, 5),
    ],
  );
}

#[test]
fn changes_lines_in_spaces() {
  run(
    "a \n b",
    vec![
      Token::new(TokenType::Word, 0, 1),
      Token::new(TokenType::Space, 1, 4),
      Token::new(TokenType::Word, 4, 5),
    ],
  );
}

#[test]
fn tokenizes_control_chars() {
  run(
    "{:;}",
    vec![
      Token::new(TokenType::OpenCurly, 0, 1),
      Token::new(TokenType::Colon, 1, 2),
      Token::new(TokenType::Semicolon, 2, 3),
      Token::new(TokenType::CloseCurly, 3, 4),
    ],
  );
}

#[test]
fn escapes_control_symbols() {
  run(
    "\\(\\{\\\"\\@\\\\\"\"",
    vec![
      Token::new(TokenType::Word, 0, 2),
      Token::new(TokenType::Word, 2, 4),
      Token::new(TokenType::Word, 4, 6),
      Token::new(TokenType::Word, 6, 8),
      Token::new(TokenType::Word, 8, 10),
      Token::new(TokenType::String, 10, 12),
    ],
  );
}

#[test]
fn escapes_backslash() {
  run(
    "\\\\\\\\{",
    vec![
      Token::new(TokenType::Word, 0, 4),
      Token::new(TokenType::OpenCurly, 4, 5),
    ],
  );
}

#[test]
fn tokenizes_simple_brackets() {
  run("(ab)", vec![Token::new(TokenType::Brackets, 0, 4)]);
}

#[test]
fn tokenizes_square_brackets() {
  run(
    "a[bc]",
    vec![
      Token::new(TokenType::Word, 0, 1),
      Token::new(TokenType::OpenSquare, 1, 2),
      Token::new(TokenType::Word, 2, 4),
      Token::new(TokenType::CloseSquare, 4, 5),
    ],
  );
}

#[test]
fn tokenizes_complicated_brackets() {
  run(
    "(())(\"\")(/**/)(\\\\)(\n)(",
    vec![
      Token::new(TokenType::OpenParentheses, 0, 1),
      Token::new(TokenType::Brackets, 1, 3),
      Token::new(TokenType::CloseParentheses, 3, 4),
      Token::new(TokenType::OpenParentheses, 4, 5),
      Token::new(TokenType::String, 5, 7),
      Token::new(TokenType::CloseParentheses, 7, 8),
      Token::new(TokenType::OpenParentheses, 8, 9),
      Token::new(TokenType::Comment, 9, 13),
      Token::new(TokenType::CloseParentheses, 13, 14),
      Token::new(TokenType::OpenParentheses, 14, 15),
      Token::new(TokenType::Word, 15, 17),
      Token::new(TokenType::CloseParentheses, 17, 18),
      Token::new(TokenType::OpenParentheses, 18, 19),
      Token::new(TokenType::Space, 19, 20),
      Token::new(TokenType::CloseParentheses, 20, 21),
      Token::new(TokenType::OpenParentheses, 21, 22),
    ],
  );
}

#[test]
fn tokenizes_string() {
  run(
    "'\"'\"\\\"\"",
    vec![
      Token::new(TokenType::String, 0, 3),
      Token::new(TokenType::String, 3, 7),
    ],
  );
}

#[test]
fn tokenizes_escaped_string() {
  run("\"\\\\\"", vec![Token::new(TokenType::String, 0, 4)]);
}

#[test]
fn changes_lines_in_strings() {
  run(
    "\"\n\n\"\"\n\n\"",
    vec![
      Token::new(TokenType::String, 0, 4),
      Token::new(TokenType::String, 4, 8),
    ],
  );
}

#[test]
fn tokenizes_at_word() {
  run(
    "@word ",
    vec![
      Token::new(TokenType::AtWord, 0, 5),
      Token::new(TokenType::Space, 5, 6),
    ],
  );
}

#[test]
fn tokenizes_at_word_end() {
  run(
    "@one{@two()@three\"\"@four;",
    vec![
      Token::new(TokenType::AtWord, 0, 4),
      Token::new(TokenType::OpenCurly, 4, 5),
      Token::new(TokenType::AtWord, 5, 9),
      Token::new(TokenType::Brackets, 9, 11),
      Token::new(TokenType::AtWord, 11, 17),
      Token::new(TokenType::String, 17, 19),
      Token::new(TokenType::AtWord, 19, 24),
      Token::new(TokenType::Semicolon, 24, 25),
    ],
  );
}

#[test]
fn tokenizes_urls() {
  run(
    "url(/*\\))",
    vec![
      Token::new(TokenType::Word, 0, 3),
      Token::new(TokenType::Brackets, 3, 9),
    ],
  );
}

#[test]
fn tokenizes_quoted_urls() {
  run(
    "url(\")\")",
    vec![
      Token::new(TokenType::Word, 0, 3),
      Token::new(TokenType::OpenParentheses, 3, 4),
      Token::new(TokenType::String, 4, 7),
      Token::new(TokenType::CloseParentheses, 7, 8),
    ],
  );
}

#[test]
fn tokenizes_urls_with_2_open_parentheses_unclosed() {
  run(
    "url(foo)(",
    vec![
      Token::new(TokenType::Word, 0, 3),
      Token::new(TokenType::Brackets, 3, 8),
      Token::new(TokenType::OpenParentheses, 8, 9),
    ],
  );
}

#[test]
fn tokenizes_at_symbol() {
  run("@", vec![Token::new(TokenType::AtWord, 0, 1)]);
}

#[test]
fn tokenizes_comment() {
  run("/* a\nb */", vec![Token::new(TokenType::Comment, 0, 9)]);
}

#[test]
fn changes_lines_in_comments() {
  run(
    "a/* \n */b",
    vec![
      Token::new(TokenType::Word, 0, 1),
      Token::new(TokenType::Comment, 1, 8),
      Token::new(TokenType::Word, 8, 9),
    ],
  );
}

#[test]
fn supports_line_feed() {
  run(
    "a\u{c}b",
    vec![
      Token::new(TokenType::Word, 0, 1),
      Token::new(TokenType::Space, 1, 2),
      Token::new(TokenType::Word, 2, 3),
    ],
  );
}

#[test]
fn supports_carriage_return() {
  run(
    "a\rb\r\nc",
    vec![
      Token::new(TokenType::Word, 0, 1),
      Token::new(TokenType::Space, 1, 2),
      Token::new(TokenType::Word, 2, 3),
      Token::new(TokenType::Space, 3, 5),
      Token::new(TokenType::Word, 5, 6),
    ],
  );
}

#[test]
fn tokenizes_css() {
  run(
    "a {\n  content: \"a\";\n  width: calc(1px;)\n  }\n/* small screen */\n@media screen {}",
    vec![
      Token::new(TokenType::Word, 0, 1),
      Token::new(TokenType::Space, 1, 2),
      Token::new(TokenType::OpenCurly, 2, 3),
      Token::new(TokenType::Space, 3, 6),
      Token::new(TokenType::Word, 6, 13),
      Token::new(TokenType::Colon, 13, 14),
      Token::new(TokenType::Space, 14, 15),
      Token::new(TokenType::String, 15, 18),
      Token::new(TokenType::Semicolon, 18, 19),
      Token::new(TokenType::Space, 19, 22),
      Token::new(TokenType::Word, 22, 27),
      Token::new(TokenType::Colon, 27, 28),
      Token::new(TokenType::Space, 28, 29),
      Token::new(TokenType::Word, 29, 33),
      Token::new(TokenType::Brackets, 33, 39),
      Token::new(TokenType::Space, 39, 42),
      Token::new(TokenType::CloseCurly, 42, 43),
      Token::new(TokenType::Space, 43, 44),
      Token::new(TokenType::Comment, 44, 62),
      Token::new(TokenType::Space, 62, 63),
      Token::new(TokenType::AtWord, 63, 69),
      Token::new(TokenType::Space, 69, 70),
      Token::new(TokenType::Word, 70, 76),
      Token::new(TokenType::Space, 76, 77),
      Token::new(TokenType::OpenCurly, 77, 78),
      Token::new(TokenType::CloseCurly, 78, 79),
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
      Token::new(TokenType::Space, 0, 1),
      Token::new(TokenType::String, 1, 3),
    ],
  );
}

#[test]
fn ignores_unclosing_comment_on_request() {
  run_ignore_errors(
    " /*",
    vec![
      Token::new(TokenType::Space, 0, 1),
      Token::new(TokenType::Comment, 1, 3),
    ],
  );
}

#[test]
fn ignores_unclosing_function_on_request() {
  run_ignore_errors(
    "url(",
    vec![
      Token::new(TokenType::Word, 0, 3),
      Token::new(TokenType::Brackets, 3, 4),
    ],
  );
}

#[test]
fn tokenizes_hexadecimal_escape() {
  run(
    "\\0a \\09 \\z ",
    vec![
      Token::new(TokenType::Word, 0, 4),
      Token::new(TokenType::Word, 4, 8),
      Token::new(TokenType::Word, 8, 10),
      Token::new(TokenType::Space, 10, 11),
    ],
  );
}

#[test]
fn ignore_unclosed_per_token_request() {
  fn token(css: &str) -> Vec<Token> {
    let input = Input::new(css, None);
    let processor = Tokenizer::new(input.css, false);
    let mut tokens = vec![];
    while !processor.end_of_file() {
      tokens.push(processor.next_token(true))
    }
    tokens
  }

  let tokens = token("How's it going (");
  let expected = vec![
    Token::new(TokenType::Word, 0, 3),
    Token::new(TokenType::String, 3, 5),
    Token::new(TokenType::Space, 5, 6),
    Token::new(TokenType::Word, 6, 8),
    Token::new(TokenType::Space, 8, 9),
    Token::new(TokenType::Word, 9, 14),
    Token::new(TokenType::Space, 14, 15),
    Token::new(TokenType::OpenParentheses, 15, 16),
  ];
  assert_eq!(tokens, expected);
}

#[test]
fn provides_correct_position() {
  let css = "Three tokens";
  let input = Input::new(css, None);
  let processor = Tokenizer::new(input.css, false);
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
