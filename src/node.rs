use crate::input::Input;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
  pub offset: usize,
  pub column: usize,
  pub line: usize,
}

impl Position {
  pub fn new(offset: usize, line: usize, column: usize) -> Self {
    Self {
      offset,
      column,
      line,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Source<'a> {
  #[serde(skip_serializing, skip_deserializing)]
  pub input: Rc<RefCell<Input<'a>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start: Option<Position>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end: Option<Position>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RawValue {
  pub value: String,
  pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Node<'a> {
  Root(Root<'a>),
  AtRule(AtRule<'a>),
  Rule(Rule<'a>),
  Decl(Declaration<'a>),
  Comment(Comment<'a>),
  Document(Document<'a>),
}

impl<'a> Node<'a> {
  pub fn as_shared_mut(&mut self) -> &mut dyn NodeTrait<'a> {
    match self {
      Node::Root(root) => root,
      Node::AtRule(at) => at,
      Node::Rule(rule) => rule,
      Node::Decl(decl) => decl,
      Node::Comment(comment) => comment,
      Node::Document(document) => document,
    }
  }

  pub fn as_shared(&self) -> &dyn NodeTrait<'a> {
    match self {
      Node::Root(root) => root,
      Node::AtRule(at) => at,
      Node::Rule(rule) => rule,
      Node::Decl(decl) => decl,
      Node::Comment(comment) => comment,
      Node::Document(document) => document,
    }
  }

  pub fn set_source(
    &mut self,
    input: Rc<RefCell<Input<'a>>>,
    start: Option<Position>,
    end: Option<Position>,
  ) {
    self
      .as_shared_mut()
      .set_source(Source { input, start, end });
  }

  pub fn set_source_end(&mut self, end: Option<Position>) {
    if let Some(source) = self.as_shared_mut().get_source().as_mut() {
      source.end = end
    }
  }

  pub fn set_raw_before(&mut self, before: String) {
    self.as_shared_mut().as_raws_mut().set_raw_before(before);
  }

  pub fn push_child(&mut self, node: Rc<RefCell<Node<'a>>>) {
    if let Some(children) = self.as_shared().get_nodes().as_mut() {
      children.push(node)
    }
  }

  pub fn as_root(&self) -> Option<&Root<'a>> {
    if let Self::Root(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_at_rule(&self) -> Option<&AtRule<'a>> {
    if let Self::AtRule(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_rule(&self) -> Option<&Rule<'a>> {
    if let Self::Rule(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_decl(&self) -> Option<&Declaration<'a>> {
    if let Self::Decl(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_comment(&self) -> Option<&Comment<'a>> {
    if let Self::Comment(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_document(&self) -> Option<&Document<'a>> {
    if let Self::Document(v) = self {
      Some(v)
    } else {
      None
    }
  }
  pub fn as_root_mut(&self) -> Option<&Root<'a>> {
    if let Self::Root(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_at_rule_mut(&mut self) -> Option<&mut AtRule<'a>> {
    if let Self::AtRule(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_rule_mut(&mut self) -> Option<&mut Rule<'a>> {
    if let Self::Rule(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_decl_mut(&mut self) -> Option<&mut Declaration<'a>> {
    if let Self::Decl(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_comment_mut(&mut self) -> Option<&mut Comment<'a>> {
    if let Self::Comment(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_document_mut(&mut self) -> Option<&mut Document<'a>> {
    if let Self::Document(v) = self {
      Some(v)
    } else {
      None
    }
  }

  /// Returns `true` if the node is [`Root`].
  ///
  /// [`Root`]: Node::Root
  pub fn is_root(&self) -> bool {
    matches!(self, Self::Root(..))
  }

  /// Returns `true` if the node is [`AtRule`].
  ///
  /// [`AtRule`]: Node::AtRule
  pub fn is_at_rule(&self) -> bool {
    matches!(self, Self::AtRule(..))
  }

  /// Returns `true` if the node is [`Rule`].
  ///
  /// [`Rule`]: Node::Rule
  pub fn is_rule(&self) -> bool {
    matches!(self, Self::Rule(..))
  }

  /// Returns `true` if the node is [`Decl`].
  ///
  /// [`Decl`]: Node::Decl
  pub fn is_decl(&self) -> bool {
    matches!(self, Self::Decl(..))
  }

  /// Returns `true` if the node is [`Comment`].
  ///
  /// [`Comment`]: Node::Comment
  pub fn is_comment(&self) -> bool {
    matches!(self, Self::Comment(..))
  }

  /// Returns `true` if the node is [`Document`].
  ///
  /// [`Document`]: Node::Document
  pub fn is_document(&self) -> bool {
    matches!(self, Self::Document(..))
  }
}

fn is_false(boolean: &bool) -> bool {
  !*boolean
}

/// CommonBehaviors
pub trait NodeTrait<'a> {
  fn get_nodes(&self) -> Option<Vec<Rc<RefCell<Node<'a>>>>>;
  fn get_source(&self) -> Option<Source<'a>>;
  fn set_source(&mut self, source: Source<'a>);
  fn as_raws(&self) -> &dyn RawBefore;
  fn as_raws_mut(&mut self) -> &mut dyn RawBefore;
  fn as_trait(&'a self) -> &dyn NodeTrait<'a>;
}

macro_rules! impl_node_traits {
  ($ident: ident) => {
    impl<'a> NodeTrait<'a> for $ident<'a> {
      fn get_nodes(&self) -> Option<Vec<Rc<RefCell<Node<'a>>>>> {
        self.nodes.clone()
      }

      fn get_source(&self) -> Option<Source<'a>> {
        self.source.clone()
      }

      fn set_source(&mut self, source: Source<'a>) {
        self.source = Some(source);
      }

      fn as_raws(&self) -> &dyn RawBefore {
        &self.raws as &dyn RawBefore
      }

      fn as_raws_mut(&mut self) -> &mut dyn RawBefore {
        &mut self.raws as &mut dyn RawBefore
      }

      fn as_trait(&'a self) -> &dyn NodeTrait<'a> {
        self as &dyn NodeTrait<'a>
      }
    }
  };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Declaration<'a> {
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
  #[serde(default)]
  #[serde(skip_serializing_if = "is_false")]
  pub important: bool,

  /// `true` if declaration is declaration of CSS Custom Property
  /// or Sass variable.
  #[serde(default)]
  #[serde(skip_serializing_if = "is_false")]
  pub variable: bool,

  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: DeclarationRaws,
}

impl_node_traits!(Declaration);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rule<'a> {
  /// Selector or selectors of the rule.
  pub selector: String,

  ///  Selectors of the rule represented as an array of strings.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub selectors: Option<Vec<String>>,
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: RuleRaws,
}
impl_node_traits!(Rule);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtRule<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `atrule` or `document`.
  /// The at-rule's name immediately follows the `@`.
  /// Or the document's name.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  /// used in `atrule`.
  /// The at-rule’s parameters, the values that follow the at-rule’s name
  /// but precede any {} block.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub params: Option<String>,

  pub raws: AtRuleRaws,
}
impl_node_traits!(AtRule);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Comment<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `comment`.
  /// The comment's text.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,

  pub raws: CommentRaws,
}
impl_node_traits!(Comment);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  /// used in `atrule` or `document`.
  /// The at-rule's name immediately follows the `@`.
  /// Or the document's name.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  // document node have no raws
  #[serde(skip_serializing)]
  pub raws: DocumentRaws,
}
impl_node_traits!(Document);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root<'a> {
  /// An array containing the node’s children.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nodes: Option<Vec<Rc<RefCell<Node<'a>>>>>,

  /// The node’s parent node.
  #[serde(skip_serializing, skip_deserializing)]
  pub parent: Option<Weak<Node<'a>>>,

  /// The input source of the node.
  /// The property is used in source map generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<Source<'a>>,

  pub raws: RootRaws,
}
impl_node_traits!(Root);

pub trait RawBefore {
  fn get_raw_before(&self) -> Option<String>;
  fn set_raw_before(&mut self, value: String);
}

macro_rules! impl_raw_before_traits {
  ($ident: ident) => {
    impl RawBefore for $ident {
      fn get_raw_before(&self) -> Option<String> {
        self.before.clone()
      }
      fn set_raw_before(&mut self, value: String) {
        self.before = Some(value)
      }
    }
  };
  ($ident: ident, unimplemented) => {
    impl RawBefore for $ident {
      fn get_raw_before(&self) -> Option<String> {
        unreachable!()
      }
      fn set_raw_before(&mut self, _value: String) {
        unreachable!()
      }
    }
  };
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code_before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code_after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
}
impl_raw_before_traits!(RootRaws, unimplemented);

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtRuleRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub params: Option<RawValue>,
}
impl_raw_before_traits!(AtRuleRaws);

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub left: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub right: Option<String>,
}
impl_raw_before_traits!(CommentRaws);

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub important: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<RawValue>,
}
impl_raw_before_traits!(DeclarationRaws);

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleRaws {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub between: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub own_semicolon: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub selector: Option<RawValue>,
}
impl_raw_before_traits!(RuleRaws);

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DocumentRaws {}
impl_raw_before_traits!(DocumentRaws, unimplemented);
