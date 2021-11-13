use cssparser::Token;

#[repr(u16)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SyntaxKind {
  // SyntaxToken
  /// (
  OpenParentheses,
  /// )
  CloseParentheses,
  Space,
  Word,
  String,
  /// [
  OpenSquare,
  /// ]
  CloseSquare,
  /// {
  OpenCurly,
  /// }
  CloseCurly,
  /// ;
  Semicolon,
  /// :
  Colon,
  AtWord,
  Brackets,

  // SyntaxNode
  Root,
  Document,
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

// pub(crate) struct Lexer<'a, 'b> {
//   inner: cssparser::Parser<'a, 'b>,
// }

// impl<'a, 'b> Lexer<'a, 'b> {
//   pub(crate) fn new(input: &'a str) -> Self {
//     Self {
//       inner: cssparser::Parser::new(input, false),
//     }
//   }
// }

// impl<'a, 'b> Iterator for Lexer<'a, 'b> {
//   type Item = (SyntaxKind, &'a str);

//   fn next(&mut self) -> Option<Self::Item> {
//     let loc = self.inner.position();
//     let token = self.inner.next_including_whitespace_and_comments();
//     println!("loc: {:?}, {:?}", loc, token);
//   }
// }

// impl<'a> From<Token<'a>> for SyntaxKind {
//   fn from(token: Token) -> Self {
//     match token {
//       Ok(token) => match token {
//         Token::Ident(_) => (),
//         Token::AtKeyword(_) => (),
//         Token::Hash(_) => (),
//         Token::IDHash(_) => (),
//         Token::QuotedString(_) => (),
//         Token::UnquotedUrl(_) => (),
//         Token::Delim(_) => (),
//         Token::Number {
//           has_sign,
//           value,
//           int_value,
//         } => (),
//         Token::Percentage {
//           has_sign,
//           unit_value,
//           int_value,
//         } => (),
//         Token::Dimension {
//           has_sign,
//           value,
//           int_value,
//           unit,
//         } => (),
//         Token::WhiteSpace(_) => (),
//         Token::Comment(_) => (),
//         Token::Colon => (),
//         Token::Semicolon => (),
//         Token::Comma => (),
//         Token::IncludeMatch => (),
//         Token::DashMatch => (),
//         Token::PrefixMatch => (),
//         Token::SuffixMatch => (),
//         Token::SubstringMatch => (),
//         Token::CDO => (),
//         Token::CDC => (),
//         Token::Function(a) => (),
//         Token::ParenthesisBlock => (),
//         Token::SquareBracketBlock => (),
//         Token::CurlyBracketBlock => (),
//         Token::BadUrl(_) => (),
//         Token::BadString(_) => (),
//         Token::CloseParenthesis => (),
//         Token::CloseSquareBracket => (),
//         Token::CloseCurlyBracket => (),
//       },
//     };
//   }
// }

// #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
// pub enum Lang {}

// impl rowan::Language for Lang {
//   type Kind = SyntaxKind;
//   fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
//     assert!(raw.0 <= SyntaxKind::Comment as u16);
//     unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
//   }
//   fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
//     kind.into()
//   }
// }
// pub type SyntaxNode = rowan::SyntaxNode<Lang>;
// pub type SyntaxToken = rowan::SyntaxToken<Lang>;
