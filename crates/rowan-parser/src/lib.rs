use syntax::SyntaxNode;

pub mod parser;
pub mod syntax;

pub fn ast_printer(root: SyntaxNode, level: usize, compatible_with_postcss: bool) {
  println!("{}{:?}", " ".repeat(level * 2), root);
  for child in root.children() {
    if !compatible_with_postcss {
      ast_printer(child, level + 1, compatible_with_postcss);
    } else {
      match child.kind() {
        syntax::SyntaxKind::Root
        | syntax::SyntaxKind::Document
        | syntax::SyntaxKind::Declaration
        | syntax::SyntaxKind::AtRule
        | syntax::SyntaxKind::Rule
        | syntax::SyntaxKind::Comment => {
          ast_printer(child, level + 1, compatible_with_postcss);
        }

        syntax::SyntaxKind::Selector
        | syntax::SyntaxKind::Params
        | syntax::SyntaxKind::Value
        | syntax::SyntaxKind::Prop => {}
        _ => {
          unreachable!()
        }
      }
    }
  }
}
