use std::any::Any;

use crate::ast::{Node, Props, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RuleRaws {
  /// The space symbols before the node. It also stores `*`
  /// and `_` symbols before the declaration (IE hack).
  pub before: Option<String>,

  /// The space symbols after the last child of the node to the end of the node.
  pub after: Option<String>,

  /// The symbols between the last parameter and `{` for rules.
  pub between: Option<String>,

  /// Contains `true` if the last child has an (optional) semicolon.
  pub semicolon: Option<bool>,

  /// Contains `true` if there is semicolon after rule.
  pub own_semicolon: Option<bool>,

  /// The rule’s selector with comments.
  pub selector: Option<RawValue>,
}

pub struct RuleProps {
  /// Selector or selectors of the rule.
  pub selector: String,

  ///  Selectors of the rule represented as an array of strings.
  pub selectors: Option<Vec<String>>,

  /// Information used to generate byte-to-byte equal node string as it was in the origin input.
  pub raws: Option<RuleRaws>,

  nodes: Option<Vec<Box<dyn Props>>>,

  source: Option<Source>,
}

/// Represents a CSS rule: a selector followed by a declaration block.
pub struct Rule {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: &'static str,

  pub nodes: Option<Vec<Box<dyn Node>>>,

  /// The node’s parent node.
  // pub parent: Option<Container>,

  /// Selector or selectors of the rule.
  pub selector: String,

  ///  Selectors of the rule represented as an array of strings.
  pub selectors: Option<Vec<String>>,

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  pub raws: RuleRaws,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source>,
}

impl Props for RuleProps {
  #[inline]
  fn name(&self) -> String {
    todo!()
  }
}

impl Node for Rule {
  #[inline]
  fn nodes(&self) -> Option<&Vec<Box<dyn Node>>> {
    self.nodes.as_ref()
  }

  #[inline]
  fn nodes_mut(&mut self) -> Option<&mut Vec<Box<dyn Node>>> {
    self.nodes.as_mut()
  }

  #[inline]
  fn as_any(&self) -> &dyn Any {
    self
  }

  #[inline]
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
