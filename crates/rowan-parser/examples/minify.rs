use memchr::memrchr;
use mimalloc_rust::*;
use rowan_parser::parser::Parser;
use rowan_parser::syntax::SyntaxKind;
use sourcemap::SourceMapBuilder;
use std::time::Instant;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = "body\n        {\n font-size: \n 12px;      \n} \n";
  // let css = include_str!("../../../assets/bootstrap.css");
  let start = Instant::now();
  let result = transform(css);
  println!("transform(total)\t{:?}", start.elapsed());
  println!("output:\t{}", result.output);
  println!("sourcemap:\t{}", result.sourcemap);
}

struct ParseResult {
  pub output: String,
  pub sourcemap: String,
}

fn transform(css: &str) -> ParseResult {
  let start = Instant::now();
  let parser = Parser::new(css);
  let root = parser.parse();
  println!("parse\t\t\t{:?}", start.elapsed());

  let start = Instant::now();
  let mut output = String::with_capacity(0);
  let mut sourcemap: Vec<u8> = vec![];
  let mut smb = SourceMapBuilder::new(None);
  let src_id = smb.add_source("stdin");
  smb.set_source_contents(src_id, Some(css));
  let mut src_line = 0;
  let mut src_col = 0;
  let mut dst_line = 0;
  let mut dst_col = 0;
  root.preorder_with_tokens().for_each(|e| match e {
    rowan::WalkEvent::Enter(n) => match n {
      rowan::NodeOrToken::Node(_) => {}
      rowan::NodeOrToken::Token(token) => {
        let src: &str = &css[token.text_range()]; // 💡 从 offset 获取 input css 片段
        let mut dst: String = src.to_string(); // 💡 复制一份 input 作为 output

        // plugin: remove space                       // 💡 插件1: 如果是 Space，则移除
        if token.kind() == SyntaxKind::Space {
          dst.clear();
        }

        // plugin: upper prop                         // 💡 插件2: 如果是 Word，并且 parent 是 Prop，则转成大写
        if token.kind() == SyntaxKind::Word && token.parent().unwrap().kind() == SyntaxKind::Prop {
          dst = dst.to_uppercase();
        }

        // plugin: upper prop                         // 💡 插件3: 如果是 Word，并且 parent 是 Value，则反转
        if token.kind() == SyntaxKind::Word && token.parent().unwrap().kind() == SyntaxKind::Value {
          dst = dst.chars().rev().collect();
        }

        // build source-map                           // 💡 生成 sourcemap 和 output
        if !dst.is_empty() {
          output.push_str(&dst);
          smb.add_raw(dst_line, dst_col, src_line, src_col, Some(src_id), None);

          // cacl next location
          let count = bytecount::count(dst.as_bytes(), b'\n') as u32;
          if count == 0 {
            dst_col += dst.len() as u32;
          } else {
            dst_line += count;
            dst_col +=
              unsafe { dst.len() - memrchr(b'\n', dst.as_bytes()).unwrap_unchecked() } as u32
          }
        }

        let count = bytecount::count(src.as_bytes(), b'\n') as u32;
        if count == 0 {
          src_col += src.len() as u32;
        } else {
          src_line += count;
          src_col += unsafe { src.len() - memrchr(b'\n', src.as_bytes()).unwrap_unchecked() } as u32
        }
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
