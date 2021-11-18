use rowan_parser::parser::Parser;
use rowan_parser::syntax::{SyntaxKind, SyntaxNode};
use sourcemap::SourceMapBuilder;
use std::time::Instant;

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = "#id\n    {\n font-size:  12px;      \n} \n";
  let parser = Parser::new(css);
  let parse = parser.parse();
  let root = SyntaxNode::new_root(parse.green_node);
  let start = Instant::now();
  let mut smb = SourceMapBuilder::new(None);
  let src_id = smb.add_source("stdin");
  smb.set_source_contents(src_id, Some(css));
  sourcemap(&root, &mut smb, Some(src_id));
  let sm = smb.into_sourcemap();
  let mut output: Vec<u8> = vec![];
  sm.to_writer(&mut output).unwrap();
  println!("sourcemap\t{:?}", start.elapsed());
  println!("{}", std::str::from_utf8(&output).unwrap());

  let mut output = String::with_capacity(0);
  minify(&root, &mut output, css);
  println!("output\t{}", output);
}

fn sourcemap(root: &SyntaxNode, sb: &mut SourceMapBuilder, src_id: Option<u32>) {
  root.preorder_with_tokens().for_each(|e| match e {
    rowan::WalkEvent::Enter(n) => match n {
      rowan::NodeOrToken::Node(_) => {}
      rowan::NodeOrToken::Token(tok) => {
        let col: u32 = tok.text_range().start().into();
        sb.add_raw(1, col, 1, col, src_id, None);
      }
    },
    rowan::WalkEvent::Leave(_) => {}
  });
}

fn minify(root: &SyntaxNode, output: &mut String, source: &str) {
  root.preorder_with_tokens().for_each(|e| match e {
    rowan::WalkEvent::Enter(n) => match n {
      rowan::NodeOrToken::Node(_) => {}
      rowan::NodeOrToken::Token(tok) => {
        if tok.kind() != SyntaxKind::Space {
          output.push_str(&source[tok.text_range()]);
        }
      }
    },
    rowan::WalkEvent::Leave(_) => {}
  });
}
