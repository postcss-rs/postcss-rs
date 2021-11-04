use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::ast::{Node, RawValue, Source};

#[derive(Debug, PartialEq, Clone)]
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

/// Represents an at-rule.
///
/// If it’s followed in the CSS by a {} block, this node will have
/// a nodes property representing its children.
pub struct AtRule {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: &'static str,

  pub nodes: Option<RefCell<Vec<Rc<Node>>>>,

  /// The node’s parent node.
  pub parent: Option<RefCell<Weak<Node>>>,

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
