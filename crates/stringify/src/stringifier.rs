#![allow(dead_code)]

use crate::get_raw_value;
use node::Node;
use std::hint::unreachable_unchecked;
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
      Node::Document(_) => {
        self.body(node);
      }

      Node::Root(root) => {
        self.body(node);
        if let Some(after) = &root.raws.after {
          (self.builder)(after, None, None);
        }
      }

      Node::Comment(comment) => {
        let left: &str = &comment
          .raws
          .left
          .clone()
          .unwrap_or_else(|| self.detect_str(node, "left", "commentLeft").into());
        let right: &str = &comment
          .raws
          .right
          .clone()
          .unwrap_or_else(|| self.detect_str(node, "right", "commentRight").into());
        (self.builder)(
          &("/*".to_string() + left + &comment.text + right + "*/"),
          Some(node),
          None,
        );
      }

      Node::Decl(decl) => {
        let between: &str = &decl
          .raws
          .between
          .clone()
          .unwrap_or_else(|| self.detect_str(node, "between", "colon").into());
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
    let semicolon = match node {
      Node::Document(_) => Some(false), // FIXME(justjavac):
      Node::Root(node) => node.raws.semicolon,
      Node::Rule(node) => node.raws.semicolon,
      Node::AtRule(node) => node.raws.semicolon,
      _ => unsafe { unreachable_unchecked() },
    };

    let semicolon = semicolon.unwrap_or_else(|| self.detect_bool(node, "semicolon", "semicolon"));

    for child in &nodes {
      let child_content = &*(**child).borrow();
      let before = match child_content {
        Node::Rule(node) => node.raws.before.clone(),
        Node::AtRule(node) => node.raws.before.clone(),
        Node::Comment(node) => node.raws.before.clone(),
        Node::Decl(node) => node.raws.before.clone(),
        _ => unsafe { unreachable_unchecked() },
      };

      let before =
        before.unwrap_or_else(|| self.detect_str(child_content, "before", "before").into());

      if !before.is_empty() {
        (self.builder)(&before, None, None);
      }
      self.stringify(
        child_content,
        last.is_none() || !Rc::ptr_eq(last.unwrap(), child) || semicolon,
      );
    }
  }

  pub(crate) fn block(&self, node: &Node, start: &str) {
    let between = match node {
      Node::Rule(node) => node.raws.between.clone(),
      Node::AtRule(node) => node.raws.between.clone(),
      _ => unsafe { unreachable_unchecked() },
    };
    let between = between.unwrap_or_else(|| self.detect_str(node, "between", "beforeOpen").into());

    (self.builder)(
      &(start.to_string() + &between + "{"),
      Some(node),
      Some("start"),
    );

    let after = match node {
      Node::Rule(node) => node.raws.between.clone(),
      Node::AtRule(node) => node.raws.between.clone(),
      _ => unsafe { unreachable_unchecked() },
    };

    let after = match node.as_shared().get_nodes() {
      Some(_) => {
        self.body(node);
        after.unwrap_or_else(|| self.detect_str(node, "after", "after").into())
      }
      None => after.unwrap_or_else(|| self.detect_str(node, "after", "emptyBody").into()),
    };

    if !after.is_empty() {
      (self.builder)(&after, None, None);
    }
    (self.builder)("}", Some(node), Some("end"));
  }

  pub(crate) fn detect_str(&self, _node: &Node, _own: &str, _detect: &str) -> &str {
    todo!()
  }

  pub(crate) fn detect_bool(&self, _node: &Node, _own: &str, _detect: &str) -> bool {
    todo!()
  }

  pub(crate) fn before_after(&self, _node: &Node, _detect: &str) -> &str {
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
