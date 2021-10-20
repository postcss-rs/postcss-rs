use crate::node::{ChildNode, ChildNodeOrProps, Source};

#[derive(Debug, PartialEq)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, PartialEq)]
pub struct AtRuleRaws {
  /// The space symbols before the node. It also stores `*`
  /// and `_` symbols before the declaration (IE hack).
  pub before: Option<String>,

  /// The space symbols after the last child of the node to the end of the node.
  pub after: Option<String>,

  /// The space between the at-rule name and its parameters.
  pub after_name: Option<String>,

  /// The symbols between the last parameter and `{` for rules.
  pub between: Option<String>,

  /// Contains `true` if the last child has an (optional) semicolon.
  pub semicolon: Option<bool>,

  /// The rule’s selector with comments.
  /// (value, raw)
  pub params: Option<RawValue>,
}

#[derive(Debug, PartialEq)]
pub struct AtRuleProps {
  /// Name of the at-rule.
  pub name: String,

  /// Parameters following the name of the at-rule.
  pub params: String, // | number

  /// Information used to generate byte-to-byte equal node string as it was in the origin input.
  pub raws: Option<AtRuleRaws>,

  nodes: Option<Vec<ChildNodeOrProps>>,

  source: Option<Source>,
}

/// Represents an at-rule.
///
/// If it’s followed in the CSS by a {} block, this node will have
/// a nodes property representing its children.
#[derive(Debug, PartialEq)]
pub struct AtRule {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: String,

  pub nodes: Option<Vec<ChildNode>>,

  /// The node’s parent node.
  // pub parent: Option<Container>,

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  pub raws: AtRuleRaws,

  /// The at-rule’s name immediately follows the `@`.
  pub name: String,

  /// The at-rule’s parameters, the values that follow the at-rule’s name
  /// but precede any {} block.
  pub params: String,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source>,
}
