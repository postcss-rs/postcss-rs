pub mod macros;
mod stringifier;

use node::Node;

use crate::stringifier::{Builder, Stringifier};

pub fn stringify(node: &Node, builder: Builder) {
  let str = Stringifier::new(builder);
  str.stringify(node, false);
}
