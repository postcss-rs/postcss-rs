use postcss::list::*;

#[test]
fn space_fn_splits_list_by_spaces() {
    assert_eq!(space("a b"), vec!["a", "b"]);
}

#[test]
fn space_fn_trims_values() {
    assert_eq!(space(" a  b "), vec!["a", "b"]);
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
}

#[test]
fn comma_fn_adds_last_empty() {
    assert_eq!(comma("a, b,"), vec!["a", "b", ""]);
}

#[test]
fn comma_fn_checks_quotes() {
    assert_eq!(comma("\"a,b\\\"\", ''"), vec!["\"a,b\\\"\"", "''"]);
}

#[test]
fn comma_fn_checks_functions() {
    assert_eq!(comma("f(,)), a(,(),)"), vec!["f(,))", "a(,(),)"]);
}

#[test]
fn comma_fn_does_not_split_on_escaped_commas() {
    assert_eq!(comma("a\\, b"), vec!["a\\, b"]);
}
