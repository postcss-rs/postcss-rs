use node::Node;
#[cfg(test)]
use pretty_assertions::assert_eq as pretty_assert_eq;
use serde_json::json;

#[cfg(test)]
mod test_basic_serialize {
  use node::{Root, RootRaws};
  use serde_json::Value;

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
    let value = serde_json::to_value(node).unwrap();
    assert_eq!(value, json!({"type": "root", "raws": {}}));
  }

  #[test]
  fn test_simple_css() {
    let json = include_str!("./fixtures/simple.json");
    let node = serde_json::from_str::<Node>(json).unwrap();
    // convert ast_node to json value
    let value = serde_json::to_value(node).unwrap();
    let value_from_string = serde_json::from_str::<Value>(json).unwrap();
    pretty_assert_eq!(value, value_from_string);
  }
}
