use rowan_parser::parser::Parser;
use rowan_parser::syntax::SyntaxNode;
use sourcemap::SourceMapBuilder;
use std::time::Instant;

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = "#id { font-size: 12px; }";
  // let css = include_str!("../../../assets/bootstrap.css");
  let parser = Parser::new(css);
  let parse = parser.parse();
  let root = SyntaxNode::new_root(parse.green_node);

  let start = Instant::now();
  let mut output = String::with_capacity(0);
  let mut smb = SourceMapBuilder::new(Some("stdin"));
  sourcemap(&root, &mut output, css, &mut smb);
  println!("sourcemap\t{:?}", start.elapsed());
  println!("{:?}", smb.into_sourcemap());
}

// {
//   version: 3,
//   sources: [ 'stdin' ],
//   names: [],
//   mappings: 'AAAA,MAAM,eAAe,EAAE',
//   file: 'stdin',
//   sourcesContent: [ '#id { font-size: 12px; }' ]
// }

fn sourcemap(root: &SyntaxNode, output: &mut String, source: &str, sb: &mut SourceMapBuilder) {
  root.children_with_tokens().for_each(|n| match n {
    rowan::NodeOrToken::Node(n) => {
      sourcemap(&n, output, source, sb);
    }
    rowan::NodeOrToken::Token(t) => {
      sb.add(0, 1, 0, 1, Some(t.text()), None);
    }
  });
}
