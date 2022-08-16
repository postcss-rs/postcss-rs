use crate::parser::Root;
use crate::{parser, visitor::Visit};
use std::fmt::Result;
use std::fmt::Write;
#[derive(Default)]
pub struct AstPrinter<W: Write> {
  level: usize,
  writer: W,
}

impl<W: Write> AstPrinter<W> {
  pub fn new(level: usize, writer: W) -> Self {
    Self { level, writer }
  }

  pub fn print<'a>(&mut self, root: &'a parser::Root<'a>) -> Result {
    self.visit_root(root)?;
    Ok(())
  }

  pub fn result(self) -> W {
    self.writer
  }
}

impl<'a, W: Write> Visit<'a, Result> for AstPrinter<W> {
  fn visit_root(&mut self, root: &parser::Root) -> Result {
    writeln!(
      self.writer,
      "{}Root@{:?}",
      " ".repeat(self.level * 2),
      root.start..root.end
    )?;
    self.level += 1;
    for child in &root.children {
      match child {
        parser::RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule)?;
        }
        parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule)?;
        }
        parser::RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl)?;
        }
      }
    }
    self.level -= 1;
    Ok(())
  }

  fn visit_rule(&mut self, rule: &parser::Rule) -> Result {
    writeln!(
      self.writer,
      "{}Rule@{:?}",
      " ".repeat(self.level * 2),
      rule.start..rule.end
    )?;
    self.level += 1;
    writeln!(
      self.writer,
      "{}selector: `{}`",
      " ".repeat(self.level * 2),
      rule.selector,
    )?;
    for child in &rule.children {
      match child {
        parser::RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule)?;
        }
        parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule)?;
        }
        parser::RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl)?;
        }
      }
    }
    self.level -= 1;
    Ok(())
  }

  fn visit_at_rule(&mut self, at_rule: &parser::AtRule) -> Result {
    writeln!(
      self.writer,
      "{}AtRule@{:?}",
      " ".repeat(self.level * 2),
      at_rule.start..at_rule.end
    )?;
    self.level += 1;
    writeln!(
      self.writer,
      "{}name: `{}`",
      " ".repeat(self.level * 2),
      at_rule.name,
    )?;
    writeln!(
      self.writer,
      "{}params: `{}`",
      " ".repeat(self.level * 2),
      at_rule.params,
    )?;
    for child in &at_rule.children {
      match child {
        parser::RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule)?;
        }
        parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule)?;
        }
        parser::RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl)?;
        }
      }
    }
    self.level -= 1;
    Ok(())
  }

  fn visit_declaration(&mut self, decl: &parser::Declaration) -> Result {
    writeln!(
      self.writer,
      "{}Declaration@{:?}",
      " ".repeat(self.level * 2),
      decl.start..decl.end
    )?;
    self.level += 1;
    writeln!(
      self.writer,
      "{}prop: `{}`",
      " ".repeat(self.level * 2),
      decl.prop,
    )?;
    writeln!(
      self.writer,
      "{}value: `{}`",
      " ".repeat(self.level * 2),
      decl.value,
    )?;
    self.level -= 1;
    Ok(())
  }
}

pub fn pretty_print_ast(root: &Root) -> String {
  let mut printer = AstPrinter::new(0, String::default());
  printer.print(&root).unwrap();
  let ast_string = printer.result();
  ast_string
}
