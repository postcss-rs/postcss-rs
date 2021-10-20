use crate::at_rule::AtRule;
use crate::node::AnyNode;

pub type Builder = fn(String, Option<AnyNode>, Option<String>);

pub(crate) struct Stringifier {
    pub builder: Builder,
}

impl Stringifier {
    pub fn new(builder: Builder) -> Stringifier {
        Stringifier { builder }
    }

    pub fn stringify(&self, node: &AnyNode, semicolon: bool) {
        match node {
            AnyNode::AtRule(node) => {
                let mut name = String::with_capacity(node.name.len() + 1);
                name.push('@');
                name.push_str(&node.name);
                let params = match &node.raws.params {
                    Some(raw) => {
                        let params = &node.params;
                        if raw.value == *params {
                            &raw.raw
                        } else {
                            params
                        }
                    }
                    None => &node.params,
                };
                match &node.raws.after_name {
                    Some(after_name) => {
                        name.push_str(&after_name);
                    }
                    None => {
                        if !params.is_empty() {
                            name.push(' ');
                        }
                    }
                };

                name.push_str(&params);

                match node.nodes {
                    Some(_) => self.block(node, &name),
                    None => {
                        node.raws.between.as_ref().map(|x| name.push_str(x));

                        if semicolon {
                            name.push(';');
                        }

                        (self.builder)(name, Some(AnyNode::AtRule(*node)), None);
                    }
                }
            }
            _ => {
                println!("Unknown AST node type. Maybe you need to change PostCSS stringifier.")
            }
        }
    }

    pub fn block(&self, _node: &AtRule, _name: &str) {
        // un-impl
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
