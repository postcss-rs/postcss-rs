use crate::parser::{AtRule, Declaration, Root, Rule};

pub trait Visit<'a, R = ()> {
  fn visit_root(&mut self, _root: &Root<'a>) -> R;
  fn visit_rule(&mut self, _rule: &Rule<'a>) -> R;
  fn visit_at_rule(&mut self, _at_rule: &AtRule<'a>) -> R;
  fn visit_declaration(&mut self, _decl: &Declaration<'a>) -> R;
}

pub trait VisitMut<'a, R = ()> {
  fn visit_root(&mut self, _root: &mut Root<'a>) -> R;
  fn visit_rule(&mut self, _rule: &mut Rule<'a>) -> R;
  fn visit_at_rule(&mut self, _at_rule: &mut AtRule<'a>) -> R;
  fn visit_declaration(&mut self, _decl: &mut Declaration<'a>) -> R;
}
