use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::ast::{Node, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct CommentRaws {
  ///  The space symbols before the node.
  pub before: Option<String>,

  /// The space symbols between `/*` and the comment’s text.
  pub left: Option<String>,

  /// The space symbols between the comment’s text.
  pub right: Option<String>,
}

/// Represents a comment between declarations or statements (rule and at-rules).
///
/// Comments inside selectors, at-rule parameters, or declaration values
/// will be stored in the `raws` properties explained above.
pub struct Comment {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: &'static str,

  pub nodes: Option<RefCell<Vec<Rc<Node>>>>,

  // / The node’s parent node.
  pub parent: Option<RefCell<Weak<Node>>>,

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  pub raws: CommentRaws,

  /// The comment's text.
  pub text: String,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source>,
}
