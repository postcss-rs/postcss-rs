use std::{borrow::Cow, time::Instant};

use mimalloc_rust::*;
use recursive_parser::{parser::*, visitor::VisitMut};

// #[global_allocator]
// static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = "#id {                     font-size: 12px; 
    width: 100px;
    @media test {
        .test { width: 100px; height: 200px;}
    }
}";
  let bootstrap = include_str!("../../../assets/bootstrap.css");
  let mut start = Instant::now();
  let mut root = Parser::new(bootstrap).parse();
  println!("parse {:?}", start.elapsed());
  start = Instant::now();
  ReverseProp::default().visit_root(&mut root);
  println!("reverse {:?}", start.elapsed());
  // SimplePrettier::default().visit_root(&mut root);
}

#[derive(Default)]
struct SimplePrettier {
  level: usize,
}

impl VisitMut for SimplePrettier {
  fn visit_root(&mut self, root: &mut Root) {
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

  fn visit_rule(&mut self, rule: &mut Rule) {
    print!(
      "{}{} {}\n",
      " ".repeat(self.level * 2),
      rule.selector.content,
      "{"
    );
    self.level += 1;
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
    self.level -= 1;
    print!("{}{}\n", " ".repeat(self.level * 2), "}");
  }

  fn visit_at_rule(&mut self, at_rule: &mut AtRule) {
    print!(
      "{}{} {} {}\n",
      " ".repeat(self.level * 2),
      at_rule.name,
      at_rule.params,
      "{"
    );
    self.level += 1;
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
          //   self.visit_declaration(decl);
        }
      });
    self.level -= 1;
    print!("{}{}\n", " ".repeat(self.level * 2), "}");
  }

  fn visit_declaration(&mut self, decl: &mut Declaration) {
    println!(
      "{}{} : {};",
      " ".repeat(self.level * 2),
      decl.prop,
      decl.value
    );
  }
}

#[derive(Default)]
struct ReverseProp {}

impl VisitMut for ReverseProp {
  fn visit_root(&mut self, root: &mut Root) {
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

  fn visit_rule(&mut self, rule: &mut Rule) {
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

  fn visit_at_rule(&mut self, at_rule: &mut AtRule) {
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

  fn visit_declaration(&mut self, decl: &mut Declaration) {
    decl.prop.content = Cow::Owned(decl.prop.content.chars().rev().collect());
  }
}
