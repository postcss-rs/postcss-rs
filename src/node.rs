use std::rc::Weak;

use crate::input::Input;
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
  pub offset: usize,
  pub column: usize,
  pub line: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Source<'a> {
  #[serde(skip_serializing)]
  pub input: &'a Input<'a>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start: Option<Position>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end: Option<Position>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Node<'a> {
  Root(Root<'a>),
  AtRule(AtRule<'a>),
  Rule(Rule<'a>),
  Decl(Declaration<'a>),
  Comment(Comment<'a>),
  Document(Document<'a>),
}

#[derive(Debug, Clone, Serialize)]
pub struct Declaration<'a> {
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

  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Node<'a>>>,

  /// The node’s parent node.
  #[serde(skip_serializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: DeclarationRaws,
}

#[derive(Debug, Clone, Serialize)]
pub struct Rule<'a> {
  /// Selector or selectors of the rule.
  pub selector: String,

  ///  Selectors of the rule represented as an array of strings.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub selectors: Option<Vec<String>>,
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Node<'a>>>,

  /// The node’s parent node.
  #[serde(skip_serializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: RuleRaws,
}

#[derive(Debug, Clone, Serialize)]
pub struct AtRule<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Node<'a>>>,

  /// The node’s parent node.
  #[serde(skip_serializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `atrule` or `document`.
  /// The at-rule's name immediately follows the `@`.
  /// Or the document's name.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  /// used in `atrule`.
  /// The at-rule’s parameters, the values that follow the at-rule’s name
  /// but precede any {} block.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub params: Option<String>,

  pub raws: AtRuleRaws,
}

#[derive(Debug, Clone, Serialize)]
pub struct Comment<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Node<'a>>>,

  /// The node’s parent node.
  #[serde(skip_serializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `comment`.
  /// The comment's text.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,

  pub raws: CommentRaws,
}
#[derive(Debug, Clone, Serialize)]
pub struct Document<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Node<'a>>>,

  /// The node’s parent node.
  #[serde(skip_serializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `atrule` or `document`.
  /// The at-rule's name immediately follows the `@`.
  /// Or the document's name.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  // document node have no raws
  // pub raws: Document
}
#[derive(Debug, Clone, Serialize)]
pub struct Root<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Node<'a>>>,

  /// The node’s parent node.
  #[serde(skip_serializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: RootRaws,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct RootRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code_before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code_after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct AtRuleRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub params: Option<RawValue>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct CommentRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub left: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub right: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct DeclarationRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub important: Option<String>,
  pub value: Option<RawValue>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct RuleRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub own_semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub selector: Option<RawValue>,
}
