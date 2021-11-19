use crate::parser::{AtRule, Declaration, Root, Rule};

pub trait Visit {
  fn visit_root(&mut self, _root: &Root) {}
  fn visit_rule(&mut self, _rule: &Rule) {}
  fn visit_at_rule(&mut self, _at_rule: &AtRule) {}
  fn visit_declaration(&mut self, _decl: &Declaration) {}
}
pub trait VisitMut {
  fn visit_root(&mut self, _root: &mut Root) {}
  fn visit_rule(&mut self, _rule: &mut Rule) {}
  fn visit_at_rule(&mut self, _at_rule: &mut AtRule) {}
  fn visit_declaration(&mut self, _decl: &mut Declaration) {}
}
