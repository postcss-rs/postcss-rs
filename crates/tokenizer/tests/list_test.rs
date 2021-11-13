use tokenizer::list::*;

#[test]
fn space_fn_splits_list_by_spaces() {
  assert_eq!(space("a b"), vec!["a", "b"]);
  assert_eq!(space("a b 你好"), vec!["a", "b", "你好"]);
}

#[test]
fn space_fn_trims_values() {
  assert_eq!(space(" a  b "), vec!["a", "b"]);
  assert_eq!(space(" a  b rust真棒 "), vec!["a", "b", "rust真棒"]);
}

#[test]
fn space_fn_checks_quotes() {
  assert_eq!(space("\"a b\\\"\" ''"), vec!["\"a b\\\"\"", "''"]);
}

#[test]
fn space_fn_checks_functions() {
  assert_eq!(space("f( )) a( () )"), vec!["f( ))", "a( () )"]);
}

#[test]
fn space_fn_does_not_split_on_escaped_spaces() {
  assert_eq!(space("a\\ b"), vec!["a\\ b"]);
}

#[test]
fn comma_fn_splits_list_by_spaces() {
  assert_eq!(comma("a, b"), vec!["a", "b"]);
  assert_eq!(comma("a, b, 你好"), vec!["a", "b", "你好"]);
}

#[test]
fn comma_fn_adds_last_empty() {
  assert_eq!(comma("a, b,"), vec!["a", "b", ""]);
  assert_eq!(comma("a, b,我你他"), vec!["a", "b", "我你他"]);
}

#[test]
fn comma_fn_checks_quotes() {
  assert_eq!(comma("\"a,b\\\"\", ''"), vec!["\"a,b\\\"\"", "''"]);
}

#[test]
fn comma_fn_checks_functions() {
  assert_eq!(comma("f(,)), a(,(),)"), vec!["f(,))", "a(,(),)"]);
  assert_eq!(comma("f(,)), a(,(你好),)"), vec!["f(,))", "a(,(你好),)"]);
}

#[test]
fn comma_fn_does_not_split_on_escaped_commas() {
  assert_eq!(comma("a\\, b"), vec!["a\\, b"]);
}
