use mimalloc_rust::*;
use recursive_parser::{parser::Parser, visitor::VisitMut};

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
  let css = "#id {                     font-size: 12px; 
    width: 100px;
    @media test {
        .test { width: 100px; height: 200px;}
    }
}";
  let mut root = Parser::new(css).parse();
  SimplePrettier::default().visit_root(&mut root);
}

#[derive(Default)]
struct SimplePrettier {
  level: usize,
}

impl VisitMut for SimplePrettier {
  fn visit_root(&mut self, root: &mut recursive_parser::parser::Root) {
    root.children.iter_mut().for_each(|child| match child {
      recursive_parser::parser::RuleOrAtRuleOrDecl::Rule(rule) => {
        self.visit_rule(rule);
      }
      recursive_parser::parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
        self.visit_at_rule(at_rule);
      }
      recursive_parser::parser::RuleOrAtRuleOrDecl::Declaration(_) => {
        unreachable!()
      }
    });
  }

  fn visit_rule(&mut self, rule: &mut recursive_parser::parser::Rule) {
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
        recursive_parser::parser::RuleOrAtRuleOrDecl::Rule(_) => {
          unreachable!()
        }
        recursive_parser::parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule);
        }
        recursive_parser::parser::RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl);
        }
      });
    self.level -= 1;
    print!("{}{}\n", " ".repeat(self.level * 2), "}");
  }

  fn visit_at_rule(&mut self, at_rule: &mut recursive_parser::parser::AtRule) {
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
        recursive_parser::parser::RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule);
        }
        recursive_parser::parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule);
        }
        recursive_parser::parser::RuleOrAtRuleOrDecl::Declaration(_decl) => {
          //   self.visit_declaration(decl);
        }
      });
    self.level -= 1;
    print!("{}{}\n", " ".repeat(self.level * 2), "}");
  }

  fn visit_declaration(&mut self, decl: &mut recursive_parser::parser::Declaration) {
    println!(
      "{}{} : {};",
      " ".repeat(self.level * 2),
      decl.prop,
      decl.value
    );
  }
}
