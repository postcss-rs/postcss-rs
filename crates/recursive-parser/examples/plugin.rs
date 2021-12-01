// use mimalloc_rust::*;
use recursive_parser::{parser::*, visitor::VisitMut};
use std::{borrow::Cow, io::Write, time::Instant};

// #[global_allocator]
// static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let _css = "#id {                     font-size: 12px; 
    width: 100px;
    @media test {
        .test { width: 100px; height: 200px;}
    }
}";
  let bootstrap = include_str!("../../../assets/bootstrap.css");
  let mut start = Instant::now();
  let mut root = Parser::new(bootstrap).parse().unwrap();
  println!("parse {:?}", start.elapsed());
  start = Instant::now();
  ReverseProp::default().visit_root(&mut root);
  println!("reverse {:?}", start.elapsed());
  let start = Instant::now();
  let mut printer = SimplePrettier::new(Vec::with_capacity(bootstrap.len()));
  printer.visit_root(&mut root).unwrap();
  println!("stringify {:?}", start.elapsed());
}

#[derive(Default)]
struct SimplePrettier<W: Write> {
  level: usize,
  writer: W,
}

impl<W: Write> SimplePrettier<W> {
  pub fn new(writer: W) -> Self {
    Self { level: 0, writer }
  }
}

impl<'a, W: std::io::Write> VisitMut<'a, std::io::Result<()>> for SimplePrettier<W> {
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
    self.writer.write(
      format!(
        "{}{} {}\n",
        " ".repeat(self.level * 2),
        rule.selector.content,
        "{"
      )
      .as_bytes(),
    )?;
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
    write!(self.writer, "{}{}\n", " ".repeat(self.level * 2), "}")?;
    Ok(())
  }

  fn visit_at_rule(&mut self, at_rule: &mut AtRule<'a>) -> std::io::Result<()> {
    write!(
      self.writer,
      "{}{} {} {}\n",
      " ".repeat(self.level * 2),
      at_rule.name,
      at_rule.params,
      "{"
    )?;
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
    write!(self.writer, "{}{}\n", " ".repeat(self.level * 2), "}")
  }

  fn visit_declaration(&mut self, decl: &mut Declaration<'a>) -> std::io::Result<()> {
    write!(
      self.writer,
      "{}{} : {};",
      " ".repeat(self.level * 2),
      decl.prop,
      decl.value
    )
  }
}

#[derive(Default)]
struct ReverseProp {}

impl<'a> VisitMut<'a> for ReverseProp {
  fn visit_root(&mut self, root: &mut Root<'a>) {
    root.children.iter_mut().for_each(|child| match child {
      RuleOrAtRuleOrDecl::Rule(rule) => {
        self.visit_rule(rule);
      }
      RuleOrAtRuleOrDecl::AtRule(at_rule) => {
        self.visit_at_rule(at_rule);
      }
      RuleOrAtRuleOrDecl::Declaration(_) => {
        unreachable!()
      }
    });
  }

  fn visit_rule(&mut self, rule: &mut Rule<'a>) {
    rule
      .children
      .iter_mut()
      .for_each(|rule_child| match rule_child {
        RuleOrAtRuleOrDecl::Rule(_) => {
          unreachable!()
        }
        RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule);
        }
        RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl);
        }
      });
  }

  fn visit_at_rule(&mut self, at_rule: &mut AtRule<'a>) {
    at_rule
      .children
      .iter_mut()
      .for_each(|rule_child| match rule_child {
        RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule);
        }
        RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule);
        }
        RuleOrAtRuleOrDecl::Declaration(_decl) => {
          unreachable!()
        }
      });
  }

  fn visit_declaration(&mut self, decl: &mut Declaration<'a>) {
    decl.prop = Cow::Owned(decl.prop.chars().rev().collect());
  }
}
