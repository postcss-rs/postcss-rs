use tokenizer::TokenType;
#[repr(u16)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SyntaxKind {
  // SyntaxToken
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
  AtWord,
  Brackets,

  // SyntaxNode
  Root,
  Declaration,
  AtRule,
  Rule,
  Comment,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
  fn from(kind: SyntaxKind) -> Self {
    Self(kind as u16)
  }
}

pub(crate) struct Lexer<'a> {
  inner: tokenizer::Tokenizer<'a>,
}

impl<'a> Lexer<'a> {
  pub(crate) fn new(input: &'a str) -> Self {
    Self {
      inner: tokenizer::Tokenizer::new(input, false),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = (SyntaxKind, &'a str);

  fn next(&mut self) -> Option<Self::Item> {
    if self.inner.end_of_file() {
      None
    } else {
      let token = self.inner.next_token(false);
      let text = &self.inner.css[token.2.unwrap()..token.3.unwrap()];

      Some((token.0.into(), text))
    }
  }
}

impl From<TokenType> for SyntaxKind {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::OpenParentheses => SyntaxKind::OpenParentheses,
            TokenType::CloseParentheses => SyntaxKind ::CloseParentheses,
            TokenType::Space => SyntaxKind::Space,
            TokenType::Word => SyntaxKind::Word,
            TokenType::String => SyntaxKind::String,
            TokenType::OpenSquare => SyntaxKind::OpenSquare,
            TokenType::CloseSquare => SyntaxKind::CloseSquare,
            TokenType::OpenCurly => SyntaxKind::OpenCurly,
            TokenType::CloseCurly => SyntaxKind::CloseCurly,
            TokenType::Semicolon => SyntaxKind::Semicolon,
            TokenType::Colon => SyntaxKind::Colon,
            TokenType::Comment => SyntaxKind::Comment,
            TokenType::AtWord => SyntaxKind::AtWord,
            TokenType::Brackets => SyntaxKind::Brackets,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Lang {}

impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Comment as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
