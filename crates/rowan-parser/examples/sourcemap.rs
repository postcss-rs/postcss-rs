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
  let mut smb = SourceMapBuilder::new(Some("stdin"));
  sourcemap(&root, css, &mut smb);
  let sm = smb.into_sourcemap();
  let mut output : Vec<u8> = vec![];
  sm.to_writer(&mut output).unwrap();
  println!("sourcemap\t{:?}", start.elapsed());
  println!("{}", std::str::from_utf8(&output).unwrap());
}

// {
//   version: 3,
//   sources: [ 'stdin' ],
//   names: [],
//   mappings: 'AAAA,MAAM,eAAe,EAAE',
//   file: 'stdin',
//   sourcesContent: [ '#id { font-size: 12px; }' ]
// }

fn sourcemap(root: &SyntaxNode, source: &str, sb: &mut SourceMapBuilder) {
  root.children_with_tokens().for_each(|n| match n {
    rowan::NodeOrToken::Node(n) => {
      sourcemap(&n, source, sb);
    }
    rowan::NodeOrToken::Token(t) => {
      sb.add(0, 1, 0, 1, Some(t.text()), None);
    }
  });
}
