use crate::stringifier::{Stringifier, Builder};
use crate::node::{AnyNode};

pub fn stringify(node: &mut AnyNode, builder: Builder) {
    let str = Stringifier::new(builder);
    str.stringify(node, false);
}
