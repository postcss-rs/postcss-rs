#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]

use crate::input::Input;
use crate::node::{self, Comment, Node, Position, Root, RootRaws, Rule, RuleRaws};
use crate::regex;
use crate::tokenizer::{Token, TokenType, Tokenizer};
use std::cell::RefCell;
use std::rc::Rc;

pub type Builder = fn(&str, Option<&Node>, Option<&str>);

pub(crate) struct Stringifier {
  pub builder: Builder,
}

impl Stringifier {
  pub fn new(builder: Builder) -> Stringifier {
    Stringifier { builder }
  }

  pub fn stringify(&self, node: &Node, semicolon: bool) {
    match node {
      Node::Document(document) => {
        self.body(node);
      }

      Node::Root(root) => {
        self.body(node);
        if let Some(after) = &root.raws.after {
          (self.builder)(after, None, None);
        }
      }

      Node::Comment(comment) => {
        let left = self.raw(node, "left", "commentLeft");
        let right = self.raw(node, "right", "commentRight");
        (self.builder)(
          &("/*".to_string() + left + &comment.text + right + "*/"),
          Some(node),
          None,
        );
      }

      Node::Decl(decl) => {
        let between = self.raw(node, "between", "colon");
        let value = self.raw_value(node, "value");
        let mut string = String::with_capacity(32);

        string.push_str(&decl.prop);
        string.push_str(between);
        string.push_str(value);

        if decl.important {
          match &decl.raws.important {
            Some(important) => string.push_str(important),
            None => string.push_str(" !important"),
          }
        }

        if semicolon {
          string.push(';');
        }

        (self.builder)(&string, Some(node), None);
      }

      Node::Rule(rule) => {
        self.block(node, self.raw_value(node, "selector"));
        if rule.raws.own_semicolon.unwrap_or(false) {
          // (self.builder)(rule.raws.own_semicolon, Some(*node), Some("end".into()));
          (self.builder)("", Some(node), Some("end"));
        }
      }

      Node::AtRule(at_rule) => {
        let mut name = String::with_capacity(32);
        name.push('@');
        name.push_str(&at_rule.name);
        let params = match &at_rule.raws.params {
          Some(raw) => {
            let params = &*at_rule.params;
            if *raw.value == *params {
              &raw.raw
            } else {
              params
            }
          }
          None => &at_rule.params,
        };
        match &at_rule.raws.after_name {
          Some(after_name) => {
            name.push_str(after_name);
          }
          None => {
            if !params.is_empty() {
              name.push(' ');
            }
          }
        };

        name.push_str(params);

        match at_rule.nodes {
          Some(_) => self.block(node, &(name + params)),
          None => {
            if let Some(ref between) = at_rule.raws.between {
              name.push_str(between);
            }

            if semicolon {
              name.push(';');
            }

            (self.builder)(&name, Some(node), None);
          }
        }
      }
      _ => {
        println!("Unknown AST node type. Maybe you need to change PostCSS stringifier.")
      }
    }
  }

  fn body(&self, node: &Node) {
    todo!()
  }

  pub fn block(&self, node: &Node, name: &str) {
    todo!()
  }

  fn raw(&self, node: &Node, arg_1: &str, arg_2: &str) -> &str {
    todo!()
  }

  fn raw_value(&self, node: &Node, arg: &str) -> &str {
    todo!()
  }
}

#[inline]
fn capitalize(s: &str) -> String {
  match s.len() {
    0 => s.to_string(),
    _ => {
      let mut res = String::with_capacity(s.len());
      res.push_str(&s[0..1].to_uppercase());
      res.push_str(&s[1..]);
      res
    }
  }
}

#[inline]
fn get_default_raw(s: &str) -> &str {
  match s {
    "colon" => ": ",
    "indent" => "    ",
    "beforeDecl" => "\n",
    "beforeRule" => "\n",
    "beforeOpen" => " ",
    "beforeClose" => "\n",
    "beforeComment" => "\n",
    "after" => "\n",
    "emptyBody" => "",
    "commentLeft" => " ",
    "commentRight" => " ",
    "semicolon" => ";", // false
    _ => "\0",
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_capitalize() {
    assert_eq!(capitalize("hello"), "Hello");
    assert_eq!(capitalize("Hello"), "Hello");
    assert_eq!(capitalize("hellO"), "HellO");
  }
}
