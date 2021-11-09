#[macro_export]
macro_rules! regex {
  ($re:literal $(,)?) => {{
    static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
    RE.get_or_init(|| regex::Regex::new($re).unwrap())
  }};
}

#[macro_export]
macro_rules! enum_mapping {
  ($value:expr, $pattern:pat => $extracted_value:expr) => {
    match $value {
      $pattern => Some($extracted_value),
      _ => None,
    }
  };
}
