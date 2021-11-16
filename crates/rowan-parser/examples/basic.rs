use rowan::TextRange;
use rowan_parser::parser::Parser;
use rowan_parser::syntax::{SyntaxKind, SyntaxNode};

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = "#id {font-size: 12px; }";
  let parser = Parser::new(css).parse();
  let root = SyntaxNode::new_root(parser.green_node);

  assert_eq!(root.kind(), SyntaxKind::Root);
  assert_eq!(root.text(), "#id {font-size: 12px; }");
  assert_eq!(root.text_range(), TextRange::new(0.into(), 23.into()));
  assert_eq!(root.children().count(), 1);

  let node1 = root.first_child().unwrap();
  assert_eq!(node1.kind(), SyntaxKind::Rule);
  assert_eq!(node1.text(), "#id {font-size: 12px; }");
  assert_eq!(node1.text_range(), TextRange::new(0.into(), 23.into()));
  assert_eq!(node1.children().count(), 2);

  {
    let mut children = node1.children();
    let node1_1 = children.next().unwrap();
    assert_eq!(node1_1.kind(), SyntaxKind::Selector);
    assert_eq!(node1_1.text(), "#id ");
    assert_eq!(node1_1.text_range(), TextRange::new(0.into(), 4.into()));
    assert_eq!(node1_1.children().count(), 0);

    let node1_2 = children.next().unwrap();
    assert_eq!(node1_2.kind(), SyntaxKind::Declaration);
    assert_eq!(node1_2.text(), "font-size: 12px");
    assert_eq!(node1_2.text_range(), TextRange::new(5.into(), 20.into()));
    assert_eq!(node1_2.children().count(), 2);

    {
      let mut children = node1_2.children();
      let node1_2_1 = children.next().unwrap();
      assert_eq!(node1_2_1.kind(), SyntaxKind::Prop);
      assert_eq!(node1_2_1.text(), "font-size");
      assert_eq!(node1_2_1.text_range(), TextRange::new(5.into(), 14.into()));
      assert_eq!(node1_2_1.children().count(), 0);

      let node1_2_2 = children.next().unwrap();
      assert_eq!(node1_2_2.kind(), SyntaxKind::Value);
      assert_eq!(node1_2_2.text(), "12px");
      assert_eq!(node1_2_2.text_range(), TextRange::new(16.into(), 20.into()));
      assert_eq!(node1_2_2.children().count(), 0);
    }
  }
}
