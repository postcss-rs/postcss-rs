use crate::node::{ChildNode, ChildNodeOrProps, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommentRaws {
  ///  The space symbols before the node.
  pub before: Option<String>,

  /// The space symbols between `/*` and the comment’s text.
  pub left: Option<String>,

  /// The space symbols between the comment’s text.
  pub right: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct CommentProps {
  /// Name of the at-rule.
  pub name: String,

  /// Content of the comment.
  pub text: String,

  /// Parameters following the name of the at-rule.
  pub params: String, // | number

  /// Information used to generate byte-to-byte equal node string as it was in the origin input.
  pub raws: Option<CommentRaws>,

  nodes: Option<Vec<ChildNodeOrProps>>,

  source: Option<Source>,
}

/// Represents a comment between declarations or statements (rule and at-rules).
///
/// Comments inside selectors, at-rule parameters, or declaration values
/// will be stored in the `raws` properties explained above.
#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: &'static str,

  pub nodes: Option<Vec<ChildNode>>,

  /// The node’s parent node.
  // pub parent: Container,

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  pub raws: CommentRaws,

  /// The comment's text.
  pub text: String,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source>,
}
