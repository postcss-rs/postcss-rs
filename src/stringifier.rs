#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::input::Input;
use crate::node::{self, Comment, Node, Position, Root, RootRaws, Rule, RuleRaws};
use crate::tokenizer::{Token, TokenType, Tokenizer};
use crate::{get_raw_value, regex};
use std::any::Any;
use std::borrow::Borrow;
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
        let left = self.raw(node, "left", Some("commentLeft"));
        let right = self.raw(node, "right", Some("commentRight"));
        (self.builder)(
          &("/*".to_string() + left + &comment.text + right + "*/"),
          Some(node),
          None,
        );
      }

      Node::Decl(decl) => {
        let between = self.raw(node, "between", Some("colon"));
        let value = get_raw_value!(decl, value);
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
        self.block(node, get_raw_value!(rule, selector));
        if rule.raws.own_semicolon.unwrap_or(false) {
          // (self.builder)(rule.raws.own_semicolon, Some(*node), Some("end".into()));
          (self.builder)("", Some(node), Some("end"));
        }
      }

      Node::AtRule(at_rule) => {
        let mut name = String::with_capacity(32);
        name.push('@');
        name.push_str(&at_rule.name);
        let params = get_raw_value!(at_rule, params);
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
      } // _ => {
        //   println!("Unknown AST node type. Maybe you need to change PostCSS stringifier.")
        // }
    }
  }

  pub(crate) fn body(&self, node: &Node) {
    let nodes = node.as_shared().get_nodes().unwrap();
    let last = nodes.iter().rfind(|&node| !(**node).borrow().is_comment());
    let semicolon = self.raw(node, "semicolon", None);
    for child in &nodes {
      let child_content = &*(**child).borrow();
      let before = self.raw(child_content, "before", None);
      if !before.is_empty() {
        (self.builder)(before, None, None);
      }
      self.stringify(
        child_content,
        last.is_none() || !Rc::ptr_eq(last.unwrap(), child) || !semicolon.is_empty(),
      );
    }
  }

  pub(crate) fn block(&self, node: &Node, start: &str) {
    let between = self.raw(node, "between", Some("beforeOpen"));
    (self.builder)(
      &(start.to_string() + between + "{"),
      Some(node),
      Some("start"),
    );

    let after = match node.as_shared().get_nodes() {
      Some(_) => {
        self.body(node);
        self.raw(node, "after", None)
      }
      None => self.raw(node, "after", Some("emptyBody")),
    };

    if !after.is_empty() {
      (self.builder)(after, None, None);
    }
    (self.builder)("}", Some(node), Some("end"));
  }

  pub(crate) fn raw(&self, node: &Node, own: &str, detect: Option<&str>) -> &str {
    let detect = detect.unwrap_or(own);
    todo!()
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
