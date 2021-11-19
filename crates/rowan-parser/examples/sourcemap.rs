use rowan_parser::parser::Parser;
use rowan_parser::syntax::SyntaxNode;
use sourcemap::SourceMapBuilder;
use std::time::Instant;

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = include_str!("../../../assets/bootstrap.css");
  let parser = Parser::new(css);
  let root = parser.parse();

  let start = Instant::now();
  let mut smb = SourceMapBuilder::new(Some("stdin"));
  let src_id = smb.add_source("stdin");
  smb.set_source_contents(src_id, Some(css));
  sourcemap(root, &mut smb, Some(src_id));
  let sm = smb.into_sourcemap();
  let mut output: Vec<u8> = vec![];
  sm.to_writer(&mut output).unwrap();
  println!("sourcemap\t{:?}", start.elapsed());
  // println!("{}", std::str::from_utf8(&output).unwrap());

  let css = "#id { font-size: 12px; }";
  let parser = Parser::new(css);
  let root = parser.parse();
  let start = Instant::now();
  let mut smb = SourceMapBuilder::new(None);
  let src_id = smb.add_source("stdin");
  smb.set_source_contents(src_id, Some(css));
  sourcemap(root, &mut smb, Some(src_id));
  let sm = smb.into_sourcemap();
  let mut output: Vec<u8> = vec![];
  sm.to_writer(&mut output).unwrap();
  println!("sourcemap\t{:?}", start.elapsed());
  println!("{}", std::str::from_utf8(&output).unwrap());
}

// postcss-js:                                        postcss-rs:
// {                                                  {
//   version: 3,                                        version: 3,
//   sources: [ 'stdin' ],                              sources: [ "stdin" ],
//   names: [],                                         names: [],
//   mappings: 'AAAA,MAAM,eAAe,EAAE',                   mappings: ";AACA,GAAG,CAAC,CAAC,CAAC,SAAS,CAAC,CAAC,IAAI,CAAC,CAAC",
//   file: 'stdin',                                     file: "stdin",
//   sourcesContent: [ '#id { font-size: 12px; }' ]     sourcesContent: [ "#id { font-size: 12px; }" ]

fn sourcemap(root: SyntaxNode, sb: &mut SourceMapBuilder, src_id: Option<u32>) {
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
