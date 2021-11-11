use std::cell::RefCell;
use std::rc::Rc;

use tokenizer::input::Input;

use self::at_rule::AtRule;
use self::comment::Comment;
use self::declaration::Declaration;
use self::document::Document;
use self::root::Root;
use self::rule::Rule;

pub mod at_rule;
pub mod comment;
pub mod declaration;
pub mod document;
pub mod root;
pub mod rule;

pub trait Props {
  fn name(&self) -> String;
}

// pub trait Node {
//   fn nodes(&self) -> Option<&Vec<Box<dyn Node>>>;
//   fn nodes_mut(&mut self) -> Option<&mut Vec<Box<dyn Node>>>;
//   fn as_any(&self) -> &dyn Any;
//   fn as_any_mut(&mut self) -> &mut dyn Any;
// }

pub enum Node {
  AtRule(AtRule),
  Comment(Comment),
  Declaration(Declaration),
  Document(Document),
  Root(Root),
  Rule(Rule),
}

impl Node {
  fn nodes(&self) -> Option<RefCell<Vec<Rc<Node>>>> {
    match self {
        Node::Root(root) => root.nodes,
        _ => None,
    }
  }
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

#[derive(Debug, PartialEq, Clone)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}
