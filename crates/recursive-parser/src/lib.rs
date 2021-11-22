use visitor::Visit;

pub mod parser;
pub mod syntax;

pub mod visitor;

#[derive(Default)]
pub struct AstPrinter {
  level: usize,
}

impl AstPrinter {
  pub fn print<'a>(&mut self, root: &'a parser::Root<'a>) {
    self.visit_root(root);
  }
}
impl<'a> Visit<'a, ()> for AstPrinter {
  fn visit_root(&mut self, root: &parser::Root) {
    println!(
      "{}Root@{:?}",
      " ".repeat(self.level * 2),
      root.start..root.end
    );
    self.level += 1;
    for child in &root.children {
      match child {
        parser::RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule);
        }
        parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule);
        }
        parser::RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl);
        }
      }
    }
    self.level -= 1;
  }

  fn visit_rule(&mut self, rule: &parser::Rule) {
    println!(
      "{}Rule@{:?}",
      " ".repeat(self.level * 2),
      rule.start..rule.end
    );
    self.level += 1;
    println!(
      "{}selector: `{}`",
      " ".repeat(self.level * 2),
      rule.selector.content,
    );
    for child in &rule.children {
      match child {
        parser::RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule);
        }
        parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule);
        }
        parser::RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl);
        }
      }
    }
    self.level -= 1;
  }

  fn visit_at_rule(&mut self, at_rule: &parser::AtRule) {
    println!(
      "{}AtRule@{:?}",
      " ".repeat(self.level * 2),
      at_rule.start..at_rule.end
    );
    self.level += 1;
    println!("{}name: `{}`", " ".repeat(self.level * 2), at_rule.name,);
    println!("{}params: `{}`", " ".repeat(self.level * 2), at_rule.params,);
    for child in &at_rule.children {
      match child {
        parser::RuleOrAtRuleOrDecl::Rule(rule) => {
          self.visit_rule(rule);
        }
        parser::RuleOrAtRuleOrDecl::AtRule(at_rule) => {
          self.visit_at_rule(at_rule);
        }
        parser::RuleOrAtRuleOrDecl::Declaration(decl) => {
          self.visit_declaration(decl);
        }
      }
    }
    self.level -= 1;
  }

  fn visit_declaration(&mut self, decl: &parser::Declaration) {
    println!(
      "{}Decl@{:?}",
      " ".repeat(self.level * 2),
      decl.start..decl.end
    );
    self.level += 1;
    println!(
      "{}prop: `{}`",
      " ".repeat(self.level * 2),
      decl.prop.content,
    );
    println!(
      "{}value: `{}`",
      " ".repeat(self.level * 2),
      decl.value.content,
    );
    self.level -= 1;
  }
}
