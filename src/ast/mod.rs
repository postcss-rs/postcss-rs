use std::any::Any;

use crate::input::Input;

use self::at_rule::AtRuleProps;
use self::comment::CommentProps;
use self::declaration::DeclarationProps;
use self::rule::RuleProps;

pub mod at_rule;
pub mod comment;
pub mod declaration;
pub mod document;
pub mod root;
pub mod rule;

pub trait Props {
  fn name(&self) -> String;
}

pub trait Node {
  fn nodes(&self) -> Option<&Vec<Box<dyn Node>>>;
  fn nodes_mut(&mut self) -> Option<&mut Vec<Box<dyn Node>>>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub enum ChildProps {
  AtRuleProps(AtRuleProps),
  RuleProps(RuleProps),
  DeclarationProps(DeclarationProps),
  CommentProps(CommentProps),
}

// pub enum ChildNodeOrProps<'a> {
//   ChildNode(ChildNode<'a>),
//   ChildProps(ChildProps<'a>),
// }

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
  /// Source offset in file. It starts from 0.
  pub offset: usize,

  /// Source line in file. In contrast to `offset` it starts from 1.
  pub column: usize,

  /// Source line in file. In contrast to `offset` it starts from 1.
  pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Source {
  /// The file source of the node.
  pub input: Input,

  /// The starting position of the node’s source.
  pub start: Option<Position>,

  /// The ending position of the node's source.
  pub end: Option<Position>,
}
