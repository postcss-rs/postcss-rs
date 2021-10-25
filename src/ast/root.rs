use crate::ast::document::Document;
use crate::node::{ChildNode, ChildNodeOrProps, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RootRaws {
  /// The space symbols after the last child of the node to the end of the node.
  pub after: Option<String>,

  /// Non-CSS code before `Root`, when `Root` is inside `Document`.
  ///
  /// **Experimental:** some aspects of this node could change within minor
  /// or patch version releases.
  pub code_before: Option<String>,

  /// Non-CSS code after `Root`, when `Root` is inside `Document`.
  ///
  /// **Experimental:** some aspects of this node could change within minor
  /// or patch version releases.
  pub code_after: Option<String>,

  /// Is the last child has an (optional) semicolon.
  pub semicolon: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub struct RootProps {
  /// Name of the root.
  pub name: String,

  /// Parameters following the name of the at-rule.
  pub params: String, // | number

  /// Information used to generate byte-to-byte equal node string as it was in the origin input.
  pub raws: Option<RootRaws>,

  nodes: Option<Vec<ChildNodeOrProps>>,

  source: Option<Source>,
}

/// Represents a CSS file and contains all its parsed nodes.
#[derive(Debug, PartialEq, Clone)]
pub struct Root {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: &'static str,

  pub nodes: Vec<ChildNode>,

  pub parent: Option<Document>,

  /// The node’s parent node.
  // pub parent: Option<Container>,

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  pub raws: Option<RootRaws>,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source>,
}

impl Root {
  pub fn new(
    nodes: Option<Vec<ChildNode>>,
    parent: Option<Document>,
    raws: Option<RootRaws>,
    source: Option<Source>,
  ) -> Self {
    Self {
      r#type: "root",
      nodes: nodes.unwrap_or(vec![]),
      parent,
      raws,
      source,
    }
  }
}
