use postcss::node::Node;
use serde_json::json;
#[cfg(test)]
mod test_basic_serialize {
  use postcss::node::{Root, RootRaws};

  use super::*;

  #[test]
  fn test_root() {
    let node = Node::Root(Root {
      nodes: None,
      parent: None,
      source: None,
      raws: RootRaws {
        after: None,
        code_before: None,
        code_after: None,
        semicolon: None,
      },
    });
    let result = serde_json::to_value(node).unwrap();
    assert_eq!(result, json!({"type": "root", "raws": {}}));
  }
}
