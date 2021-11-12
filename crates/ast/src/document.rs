use crate::ast::{Node, Source};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq, Clone)]
pub struct DocumentRaws {}

/// Represents a file and contains all its parsed nodes.
///
/// **Experimental:** some aspects of this node could change within minor
/// or patch version releases.
pub struct Document {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: &'static str,

  pub nodes: Option<RefCell<Vec<Rc<Node>>>>,

  /// The node’s parent node.
  pub parent: Option<RefCell<Weak<Node>>>,

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  pub raws: DocumentRaws,
  pub name: String,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source>,
}
