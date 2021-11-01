use crate::ast::{Node, Source, RawValue};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq, Clone)]
pub struct DeclarationRaws {
  /// The space symbols before the node. It also stores `*`
  /// and `_` symbols before the declaration (IE hack).
  pub before: Option<String>,

  /// The symbols between the last parameter and `{` for rules.
  pub between: Option<String>,

  /// The content of the important statement, if it is not just `!important`.
  pub important: Option<String>,

  /// Declaration value with comments.
  /// (value, raw)
  pub value: Option<RawValue>,
}

/// Represents a CSS declaration.
pub struct Declaration {
  /// tring representing the node’s type. Possible values are `root`, `atrule`,
  /// `rule`, `decl`, or `comment`.
  pub r#type: &'static str,
  pub parent: Option<RefCell<Weak<Node>>>,
  pub nodes: Option<RefCell<Vec<Rc<Node>>>>,

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

  /// Information to generate byte-to-byte equal node string as it was
  /// in the origin input.
  pub raws: DeclarationRaws,

  /// The input source of the node.
  /// The property is used in source map generation.
  pub source: Option<Source>,
}
