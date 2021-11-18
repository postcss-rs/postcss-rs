use rowan::{NodeOrToken, TextRange};
use rowan_parser::parser::Parser;
use rowan_parser::syntax::{SyntaxKind, SyntaxNode};

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = "#id { font-size: 12px; }";
  let parser = Parser::new(css).parse();
  let root = SyntaxNode::new_root(parser.green_node);
  let root_mut = root.clone_for_update().clone();
  remove_space_mut(&root_mut);
  let mut output = String::with_capacity(0);
  remove_space(&root, &mut output, css);

  println!("{:#?}", root);
  println!("{:#?}", root_mut);
  // [root]                           [root_mut]
  // Root@0..24                       Root@0..20
  //   Rule@0..24                       Rule@0..20
  //     Selector@0..4                    Selector@0..3
  //       Word@0..3 "#id"                  Word@0..3 "#id"
  //       Space@3..4 " "
  //     OpenCurly@4..5 "{"               OpenCurly@3..4 "{"
  //     Space@5..6 " "
  //     Declaration@6..21                Declaration@4..18
  //       Prop@6..15                       Prop@4..13
  //         Word@6..15 "font-size"           Word@4..13 "font-size"
  //       Colon@15..16 ":"                 Colon@13..14 ":"
  //       Space@16..17 " "
  //       Value@17..21                     Value@14..18
  //         Word@17..21 "12px"               Word@14..18 "12px"
  //     Semicolon@21..22 ";"             Semicolon@18..19 ";"
  //     Space@22..23 " "
  //     CloseCurly@23..24 "}"            CloseCurly@19..20 "}"

  assert_eq!(root.text(), "#id { font-size: 12px; }");
  assert_eq!(root_mut.text(), "#id{font-size:12px;}");
  assert_eq!(output, "#id{font-size:12px;}");
  assert_eq!(root.text_range(), TextRange::new(0.into(), 24.into()));
  assert_eq!(root_mut.text_range(), TextRange::new(0.into(), 20.into()));
}

fn remove_space_mut(node: &SyntaxNode) {
  for child in node.children_with_tokens() {
    if child.kind() == SyntaxKind::Space {
      child.detach();
    }
    child.as_node().map(|n| remove_space_mut(n));
  }
}

fn remove_space(node: &SyntaxNode, output: &mut String, source: &str) {
  node.children_with_tokens().for_each(|n| match n {
    NodeOrToken::Node(n) => {
      remove_space(&n, output, source);
    }
    NodeOrToken::Token(t) => {
      if t.kind() != SyntaxKind::Space {
        output.push_str(&source[t.text_range()]);
      }
    }
  });
}
