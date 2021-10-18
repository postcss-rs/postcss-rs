use postcss::input::Input;
use postcss::tokenizer::*;

fn tokenize(css: &str, ignore_errors: bool) -> Vec<Token> {
    let input = Input::new(css.to_string(), None);
    let mut processor = Tokenizer::new(input, ignore_errors);
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
        vec![Token::new("space", "\r\n \u{12}\t", None, None)],
    );
}

#[test]
fn tokenizes_word() {
    run("ab", vec![Token::new("word", "ab", Some(0), Some(1))]);
}

#[test]
fn splits_word_by_exclamation_mark() {
    run(
        "aa!bb",
        vec![
            Token::new("word", "aa", Some(0), Some(1)),
            Token::new("word", "!bb", Some(2), Some(4)),
        ],
    );
}

#[test]
fn changes_lines_in_spaces() {
    run(
        "a \n b",
        vec![
            Token::new("word", "a", Some(0), Some(0)),
            Token::new("space", " \n ", None, None),
            Token::new("word", "b", Some(4), Some(4)),
        ],
    );
}

#[test]
fn tokenizes_control_chars() {
    run(
        "{:;}",
        vec![
            Token::new("{", "{", Some(0), None),
            Token::new(":", ":", Some(1), None),
            Token::new(";", ";", Some(2), None),
            Token::new("}", "}", Some(3), None),
        ],
    );
}

#[test]
fn escapes_control_symbols() {
    run(
        "\\(\\{\\\"\\@\\\\\"\"",
        vec![
            Token::new("word", "\\(", Some(0), Some(1)),
            Token::new("word", "\\{", Some(2), Some(3)),
            Token::new("word", "\\\"", Some(4), Some(5)),
            Token::new("word", "\\@", Some(6), Some(7)),
            Token::new("word", "\\\\", Some(8), Some(9)),
            Token::new("string", "\"\"", Some(10), Some(11)),
        ],
    );
}

#[test]
fn escapes_backslash() {
    run(
        "\\\\\\\\{",
        vec![
            Token::new("word", "\\\\\\\\", Some(0), Some(3)),
            Token::new("{", "{", Some(4), None),
        ],
    );
}

#[test]
fn tokenizes_simple_brackets() {
    run(
        "(ab)",
        vec![Token::new("brackets", "(ab)", Some(0), Some(3))],
    );
}

#[test]
fn tokenizes_square_brackets() {
    run(
        "a[bc]",
        vec![
            Token::new("word", "a", Some(0), Some(0)),
            Token::new("[", "[", Some(1), None),
            Token::new("word", "bc", Some(2), Some(3)),
            Token::new("]", "]", Some(4), None),
        ],
    );
}

#[test]
fn tokenizes_complicated_brackets() {
    run(
        "(())(\"\")(/**/)(\\\\)(\n)(",
        vec![
            Token::new("(", "(", Some(0), None),
            Token::new("brackets", "()", Some(1), Some(2)),
            Token::new(")", ")", Some(3), None),
            Token::new("(", "(", Some(4), None),
            Token::new("string", "\"\"", Some(5), Some(6)),
            Token::new(")", ")", Some(7), None),
            Token::new("(", "(", Some(8), None),
            Token::new("comment", "/**/", Some(9), Some(12)),
            Token::new(")", ")", Some(13), None),
            Token::new("(", "(", Some(14), None),
            Token::new("word", "\\\\", Some(15), Some(16)),
            Token::new(")", ")", Some(17), None),
            Token::new("(", "(", Some(18), None),
            Token::new("space", "\n", None, None),
            Token::new(")", ")", Some(20), None),
            Token::new("(", "(", Some(21), None),
        ],
    );
}

#[test]
fn tokenizes_string() {
    run(
        "'\"'\"\\\"\"",
        vec![
            Token::new("string", "'\"'", Some(0), Some(2)),
            Token::new("string", "\"\\\"\"", Some(3), Some(6)),
        ],
    );
}

#[test]
fn tokenizes_escaped_string() {
    run(
        "\"\\\\\"",
        vec![Token::new("string", "\"\\\\\"", Some(0), Some(3))],
    );
}

#[test]
fn changes_lines_in_strings() {
    run(
        "\"\n\n\"\"\n\n\"",
        vec![
            Token::new("string", "\"\n\n\"", Some(0), Some(3)),
            Token::new("string", "\"\n\n\"", Some(4), Some(7)),
        ],
    );
}

#[test]
fn tokenizes_at_word() {
    run(
        "@word ",
        vec![
            Token::new("at-word", "@word", Some(0), Some(4)),
            Token::new("space", " ", None, None),
        ],
    );
}

#[test]
fn tokenizes_at_word_end() {
    run(
        "@one{@two()@three\"\"@four;",
        vec![
            Token::new("at-word", "@one", Some(0), Some(3)),
            Token::new("{", "{", Some(4), None),
            Token::new("at-word", "@two", Some(5), Some(8)),
            Token::new("brackets", "()", Some(9), Some(10)),
            Token::new("at-word", "@three", Some(11), Some(16)),
            Token::new("string", "\"\"", Some(17), Some(18)),
            Token::new("at-word", "@four", Some(19), Some(23)),
            Token::new(";", ";", Some(24), None),
        ],
    );
}

#[test]
fn tokenizes_urls() {
    run(
        "url(/*\\))",
        vec![
            Token::new("word", "url", Some(0), Some(2)),
            Token::new("brackets", "(/*\\))", Some(3), Some(8)),
        ],
    );
}

#[test]
fn tokenizes_quoted_urls() {
    run(
        "url(\")\")",
        vec![
            Token::new("word", "url", Some(0), Some(2)),
            Token::new("(", "(", Some(3), None),
            Token::new("string", "\")\"", Some(4), Some(6)),
            Token::new(")", ")", Some(7), None),
        ],
    );
}

#[test]
fn tokenizes_at_symbol() {
    run("@", vec![Token::new("at-word", "@", Some(0), Some(0))]);
}

#[test]
fn tokenizes_comment() {
    run(
        "/* a\nb */",
        vec![Token::new("comment", "/* a\nb */", Some(0), Some(8))],
    );
}

#[test]
fn changes_lines_in_comments() {
    run(
        "a/* \n */b",
        vec![
            Token::new("word", "a", Some(0), Some(0)),
            Token::new("comment", "/* \n */", Some(1), Some(7)),
            Token::new("word", "b", Some(8), Some(8)),
        ],
    );
}

#[test]
fn supports_line_feed() {
    run(
        "a\u{12}b",
        vec![
            Token::new("word", "a", Some(0), Some(0)),
            Token::new("space", "\u{12}", None, None),
            Token::new("word", "b", Some(2), Some(2)),
        ],
    );
}

#[test]
fn supports_carriage_return() {
    run(
        "a\rb\r\nc",
        vec![
            Token::new("word", "a", Some(0), Some(0)),
            Token::new("space", "\r", None, None),
            Token::new("word", "b", Some(2), Some(2)),
            Token::new("space", "\r\n", None, None),
            Token::new("word", "c", Some(5), Some(5)),
        ],
    );
}

#[test]
fn tokenizes_css() {
    run(
        "a {\n  content: \"a\";\n  width: calc(1px;)\n  }\n/* small screen */\n@media screen {}",
        vec![
            Token::new("word", "a", Some(0), Some(0)),
            Token::new("space", " ", None, None),
            Token::new("{", "{", Some(2), None),
            Token::new("space", "\n  ", None, None),
            Token::new("word", "content", Some(6), Some(12)),
            Token::new(":", ":", Some(13), None),
            Token::new("space", " ", None, None),
            Token::new("string", "\"a\"", Some(15), Some(17)),
            Token::new(";", ";", Some(18), None),
            Token::new("space", "\n  ", None, None),
            Token::new("word", "width", Some(22), Some(26)),
            Token::new(":", ":", Some(27), None),
            Token::new("space", " ", None, None),
            Token::new("word", "calc", Some(29), Some(32)),
            Token::new("brackets", "(1px;)", Some(33), Some(38)),
            Token::new("space", "\n  ", None, None),
            Token::new("}", "}", Some(42), None),
            Token::new("space", "\n", None, None),
            Token::new("comment", "/* small screen */", Some(44), Some(61)),
            Token::new("space", "\n", None, None),
            Token::new("at-word", "@media", Some(63), Some(68)),
            Token::new("space", " ", None, None),
            Token::new("word", "screen", Some(70), Some(75)),
            Token::new("space", " ", None, None),
            Token::new("{", "{", Some(77), None),
            Token::new("}", "}", Some(78), None),
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
            Token::new("space", " ", None, None),
            Token::new("string", "\"", Some(1), Some(2)),
        ],
    );
}

#[test]
fn ignores_unclosing_comment_on_request() {
    run_ignore_errors(
        " /*",
        vec![
            Token::new("space", " ", None, None),
            Token::new("comment", "/*", Some(1), Some(3)),
        ],
    );
}

#[test]
fn ignores_unclosing_function_on_request() {
    run_ignore_errors(
        "url(",
        vec![
            Token::new("word", "url", Some(0), Some(2)),
            Token::new("brackets", "(", Some(3), Some(3)),
        ],
    );
}

#[test]
fn tokenizes_hexadecimal_escape() {
    run(
        "\\0a \\09 \\z ",
        vec![
            Token::new("word", "\\0a ", Some(0), Some(3)),
            Token::new("word", "\\09 ", Some(4), Some(7)),
            Token::new("word", "\\z", Some(8), Some(9)),
            Token::new("space", " ", None, None),
        ],
    );
}

#[test]
fn ignore_unclosed_per_token_request() {
    fn tokn(css: &str) -> Vec<Token> {
        let input = Input::new(css.to_string(), None);
        let mut processor = Tokenizer::new(input, false);
        let mut tokens = vec![];
        while !processor.end_of_file() {
            tokens.push(processor.next_token(true).unwrap())
        }
        return tokens;
    }

    let tokens = tokn("How's it going (");
    let expected = vec![
        Token::new("word", "How", Some(0), Some(2)),
        Token::new("string", "'s", Some(3), Some(4)),
        Token::new("space", " ", None, None),
        Token::new("word", "it", Some(6), Some(7)),
        Token::new("space", " ", None, None),
        Token::new("word", "going", Some(9), Some(13)),
        Token::new("space", " ", None, None),
        Token::new("(", "(", Some(15), None),
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn provides_correct_position() {
    let css = "Three tokens";
    let input = Input::new(css.to_string(), None);
    let mut processor = Tokenizer::new(input, false);
    assert_eq!(processor.position(), 0);
    processor.next_token(false);
    assert_eq!(processor.position(), 5);
    processor.next_token(false);
    assert_eq!(processor.position(), 6);
    processor.next_token(false);
    assert_eq!(processor.position(), 12);
    processor.next_token(false);
    assert_eq!(processor.position(), 12);
}
