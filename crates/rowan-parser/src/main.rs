use rowan_parser::parser::Parser;
use rowan_parser::syntax::{SyntaxKind, SyntaxNode};
use std::time::Instant;

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = include_str!("../../../assets/bootstrap.css");
  //   let css = r#"/**
  // * Paste or drop some CSS here and explore
  // * the syntax tree created by chosen parser.
  // * Enjoy!
  // */
  // @media screen and (min-width: 480px) {
  //     body, resulkt, .result {
  //         background-color: lightgreen;
  //     }
  // }

  // #main {
  //     border: 1px solid black;
  // }

  // ul li {
  //   padding: 5px;
  // }

  // "#;

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
  reverse_plugin(lang, &mut output, css);
  println!("reverse plugin\t{:?}", start.elapsed());
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
