use rowan_parser::parser::Parser;
use rowan_parser::syntax::{SyntaxKind, SyntaxNode};
use std::time::Instant;

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let long_css = include_str!("../../../assets/bootstrap.css");
  let _short_css = r#"/**
  * Paste or drop some CSS here and explore
  * the syntax tree created by chosen parser.
  * Enjoy!
  */
  @media screen and (min-width: 480px) {
      body, resulkt, .result {
          background-color: lightgreen;
      }
  }

  #main {
      border: 1px solid black;
  }

  ul li {
    padding: 5px;
  }

  "#;
  let css = long_css;
  let instant = Instant::now();
  let parser = Parser::new(css);
  let node = parser.parse().green_node;
  let lang = SyntaxNode::new_root(node);
  println!("parse\t\t{:?}", instant.elapsed());

  let start = Instant::now();
  let result = format!("{}", lang);
  assert_eq!(result, css);
  println!("stringify\t{:?}", start.elapsed());

  let start = Instant::now();
  let mut output = String::with_capacity(0);
  reverse_plugin(lang.clone(), &mut output, css);
  println!("reverse plugin\t{:?}", start.elapsed());

  let start = Instant::now();
  let root_mut = lang.clone_for_update();
  remove_space(&root_mut);
  println!("remove_space\t{:?}", start.elapsed());
}

fn reverse_plugin(root: SyntaxNode, output: &mut String, source: &str) {
  root.children_with_tokens().for_each(|n| match n {
    rowan::NodeOrToken::Node(n) => {
      if n.kind() == SyntaxKind::Prop {
        output.push_str(&source[n.text_range()].chars().rev().collect::<String>());
      } else {
        reverse_plugin(n, output, source);
      }
    }
    rowan::NodeOrToken::Token(t) => output.push_str(&source[t.text_range()]),
  });
}

fn remove_space(node: &SyntaxNode) {
  for child in node.children_with_tokens() {
    if child.kind() == SyntaxKind::Space {
      child.detach();
    }
    child.as_node().map(remove_space);
  }
}
