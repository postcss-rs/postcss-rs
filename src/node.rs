use std::rc::Weak;

use crate::input::Input;

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
  pub offset: usize,
  pub column: usize,
  pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Source<'a> {
  pub input: &'a Input<'a>,
  pub start: Option<Position>,
  pub end: Option<Position>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, Clone)]
pub enum AstNodeType{
  Root,
  AtRule,
  Rule,
  Decl,
  Comment
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: AstNodeType,

  /// An array containing the node’s children.
  pub nodes: Option<Vec<Node<'a>>>,

  /// The node’s parent node.
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source<'a>>,

  /// used in `atrule` or `document`.
  /// The at-rule's name immediately follows the `@`.
  /// Or the document's name.
  pub name: Option<String>,

  /// used in `atrule`.
  /// The at-rule’s parameters, the values that follow the at-rule’s name
  /// but precede any {} block.
  pub params: Option<String>,

  /// used in `comment`.
  /// The comment's text.
  pub text: Option<String>,

  pub decl: Option<Declaration>,

  pub rule: Option<Rule>,

  pub raws: Raws,
}

#[derive(Debug, Clone)]
pub struct Declaration {
  /// The declaration's property name.
  pub prop: String,

  /// The declaration’s value.
  ///
  /// This value will be cleaned of comments. If the source value contained
  /// comments, those comments will be available in the `raws` property.
  /// If you have not changed the value, the result of `decl.toString()`
  /// will include the original raws value (comments and all).
  pub value: String,

  /// `true` if the declaration has an `!important` annotation.
  pub important: bool,

  /// `true` if declaration is declaration of CSS Custom Property
  /// or Sass variable.
  pub variable: bool,
}

#[derive(Debug, Clone)]
pub struct Rule {
  /// Selector or selectors of the rule.
  pub selector: String,

  ///  Selectors of the rule represented as an array of strings.
  pub selectors: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RootRaws {
  pub after: Option<String>,
  pub code_before: Option<String>,
  pub code_after: Option<String>,
  pub semicolon: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AtRuleRaws {
  pub before: Option<String>,
  pub after: Option<String>,
  pub after_name: Option<String>,
  pub between: Option<String>,
  pub semicolon: Option<bool>,
  pub params: Option<RawValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommentRaws {
  pub before: Option<String>,
  pub left: Option<String>,
  pub right: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclarationRaws {
  pub before: Option<String>,
  pub between: Option<String>,
  pub important: Option<String>,
  pub value: Option<RawValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RuleRaws {
  pub before: Option<String>,
  pub after: Option<String>,
  pub between: Option<String>,
  pub semicolon: Option<bool>,
  pub own_semicolon: Option<bool>,
  pub selector: Option<RawValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Raws {
  AtRuleRaws(AtRuleRaws),
  RootRaws(RootRaws),
  CommentRaws(CommentRaws),
  DeclarationRaws(DeclarationRaws),
  RuleRaws(RuleRaws),
  DocumentRaws,
}
