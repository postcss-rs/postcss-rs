use rowan_parser::parser::Parser;
use rowan_parser::syntax::{SyntaxKind, SyntaxNode};
use sourcemap::SourceMapBuilder;
use std::time::Instant;

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = "#id\n    {\n font-size:  12px;      \n} \n";
  // let css = include_str!("../../../assets/bootstrap.css");
  let start = Instant::now();
  let result = transform(css);
  println!("transform(total)\t{:?}", start.elapsed());
  println!("output: {}", result.output);
  println!("sourcemap: {}", result.sourcemap);
}

struct ParseResult {
  pub output: String,
  pub sourcemap: String,
}

fn transform(css: &str) -> ParseResult {
  let start = Instant::now();
  let parser = Parser::new(css);
  // let parse = parser.parse();
  let (node, map) = parser.parse();
  let root = SyntaxNode::new_root(node);
  println!("parse with location\t{:?}", start.elapsed());

  let start = Instant::now();
  let mut output = String::with_capacity(0);
  let mut sourcemap: Vec<u8> = vec![];
  let mut smb = SourceMapBuilder::new(None);
  let src_id = smb.add_source("stdin");
  smb.set_source_contents(src_id, Some(css));
  root.preorder_with_tokens().for_each(|e| match e {
    rowan::WalkEvent::Enter(n) => match n {
      rowan::NodeOrToken::Node(_) => {}
      rowan::NodeOrToken::Token(token) => {
        if token.kind() != SyntaxKind::Space {
          output.push_str(&css[token.text_range()]);
        }
        let offset: usize = token.text_range().start().into();
        let (src_line, src_col) = *map.get(&offset).unwrap();
        smb.add_raw(1, 1, src_line, src_col, Some(src_id), None);
      }
    },
    rowan::WalkEvent::Leave(_) => {}
  });
  let sm = smb.into_sourcemap();
  sm.to_writer(&mut sourcemap).unwrap();
  println!("outpout with sourcemap\t{:?}", start.elapsed());

  ParseResult {
    output,
    sourcemap: String::from_utf8(sourcemap).unwrap(),
  }
}
