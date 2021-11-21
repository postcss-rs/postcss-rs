use crate::parser::{AtRule, Declaration, Root, Rule};

pub trait Visit<R = ()> {
  fn visit_root(&mut self, _root: &Root) -> R;
  fn visit_rule(&mut self, _rule: &Rule) -> R;
  fn visit_at_rule(&mut self, _at_rule: &AtRule) -> R;
  fn visit_declaration(&mut self, _decl: &Declaration) -> R;
}

pub trait VisitMut<R = ()> {
  fn visit_root(&mut self, _root: &mut Root) -> R;
  fn visit_rule(&mut self, _rule: &mut Rule) -> R;
  fn visit_at_rule(&mut self, _at_rule: &mut AtRule) -> R;
  fn visit_declaration(&mut self, _decl: &mut Declaration) -> R;
}
