use crate::node::AnyNode;
use crate::stringifier::{Builder, Stringifier};

pub fn stringify(node: &mut AnyNode, builder: Builder) {
  let str = Stringifier::new(builder);
  str.stringify(node, false);
}
