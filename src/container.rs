use crate::node::{ChildNodeOrProps, Source};

#[derive(Debug, PartialEq)]
pub struct ValueOptions {
  /// An array of property names.
  pub props: Option<Vec<String>>,

  /// String that’s used to narrow down values and speed up the regexp search.
  pub fast: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct ContainerProps {
  pub nodes: Option<Vec<ChildNodeOrProps>>,
  pub source: Option<Source>,
}

#[derive(Debug, PartialEq)]
pub struct Container {
  /// An array containing the container’s children.
  // nodes: Vec<Node>,

  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: String,

  /// The node’s parent node.
  // pub parent: Option<Container>,

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  // pub raws: AtRuleRaws,

  /// The at-rule’s name immediately follows the `@`.
  name: String,

  /// The at-rule’s parameters, the values that follow the at-rule’s name
  /// but precede any {} block.
  params: String,

  /// The input source of the node.
  /// The property is used in source map generation.
  source: Option<Source>,
}
