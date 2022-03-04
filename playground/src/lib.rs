#![allow(clippy::unused_unit)]
mod utils;

use recursive_parser::{AstPrinter, WrapString};
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
pub fn ast(source: String) -> String {
  let parser = recursive_parser::parser::Parser::new(&source);

  match parser.parse() {
    Ok(root) => {
      let mut printer = AstPrinter::new(0, WrapString::default());
      printer.print(&root).unwrap();
      printer.result().0
    }
    Err(_) => "".to_string(),
  }
}

#[wasm_bindgen]
pub fn gen(source: String) -> String {
  let parser = recursive_parser::parser::Parser::new(&source);

  match parser.parse() {
    Ok(mut root) => {
      let mut minimize = MinimizePlugin::new(Vec::with_capacity(source.len()));
      minimize.visit_root(&mut root).unwrap();
      String::from_utf8(minimize.writer).unwrap_or_else(|_| "".to_string())
    }
    Err(_) => "".to_string(),
  }
}

#[derive(Default)]
struct MinimizePlugin<W: Write> {
  level: usize,
  writer: W,
}

impl<W: Write> MinimizePlugin<W> {
  pub fn new(writer: W) -> Self {
    Self { level: 0, writer }
  }
}

impl<'a, W: std::io::Write> VisitMut<'a, std::io::Result<()>> for MinimizePlugin<W> {
  fn visit_root(&mut self, root: &mut Root<'a>) -> std::io::Result<()> {
    for child in root.children.iter_mut() {
      match child {
        RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule)?;
        }
        RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule)?;
        }
        RuleOrAtRuleOrDecl::Declaration(_) => {
          unreachable!()
        }
      }
    }
    Ok(())
  }

  fn visit_rule(&mut self, rule: &mut Rule<'a>) -> std::io::Result<()> {
    self
      .writer
      .write(format!("{} {}", rule.selector.replace('\n', ""), "{").as_bytes())?;
    self.level += 1;
    for child in rule.children.iter_mut() {
      match child {
        RuleOrAtRuleOrDecl::Rule(_) => {
          unreachable!()
        }
        RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule)?;
        }
        RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl)?;
        }
      }
    }
    self.level -= 1;
    write!(self.writer, "}}")?;
    Ok(())
  }

  fn visit_at_rule(&mut self, at_rule: &mut AtRule<'a>) -> std::io::Result<()> {
    write!(self.writer, "@{} {} {{", at_rule.name, at_rule.params)?;
    self.level += 1;
    for child in at_rule.children.iter_mut() {
      match child {
        RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule)?;
        }
        RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule)?;
        }
        RuleOrAtRuleOrDecl::Declaration(_decl) => {
          //   self.visit_declaration(decl);
        }
      }
    }
    self.level -= 1;
    write!(self.writer, "}}")
  }

  fn visit_declaration(&mut self, decl: &mut Declaration<'a>) -> std::io::Result<()> {
    write!(self.writer, "{}:{};", decl.prop, decl.value)
  }
}