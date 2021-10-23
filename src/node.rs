use crate::at_rule::{AtRule, AtRuleProps};
use crate::input::Input;

#[derive(Debug, PartialEq, Clone)]
pub enum ChildNode {
  AtRule(AtRule),
  // Rule(Rule),
  // Declaration(Declaration),
  // Comment(Comment),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AnyNode {
  AtRule(AtRule),
  // Rule(Rule),
  // Declaration(Declaration),
  // Comment(Comment),
  // Root(Root),
  // Document(Document),
}

#[derive(Debug, PartialEq)]
pub enum ChildProps {
  AtRuleProps(AtRuleProps),
  // RuleProps(RuleProps),
  // DeclarationProps(DeclarationProps),
  // CommentProps(CommentProps),
}

#[derive(Debug, PartialEq)]
pub enum ChildNodeOrProps {
  ChildNode(ChildNode),
  ChildProps(ChildProps),
}

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
