use crate::ast::root::Root;
use crate::ast::{Node, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DocumentRaws {}

pub struct DocumentProps {
  /// Name of the document.
  pub name: String,

  /// Parameters following the name of the at-rule.
  pub params: String, // | number

  /// Information used to generate byte-to-byte equal node string as it was in the origin input.
  pub raws: Option<DocumentRaws>,

  nodes: Option<Vec<Box<Root>>>,

  source: Option<Source>,
}

/// Represents a file and contains all its parsed nodes.
///
/// **Experimental:** some aspects of this node could change within minor
/// or patch version releases.
pub struct Document {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: &'static str,

  pub nodes: Option<Vec<Box<dyn Node>>>,

  /// The node’s parent node.
  // pub parent: Option<Container>,

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  pub raws: DocumentRaws,
  pub name: String,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source>,
}
