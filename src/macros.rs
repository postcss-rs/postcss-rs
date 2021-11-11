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

#[macro_export]
macro_rules! impl_node_traits {
  ($ident: ident) => {
    impl<'a> NodeTrait<'a> for $ident<'a> {
      fn get_nodes(&self) -> Option<Vec<Rc<RefCell<Node<'a>>>>> {
        self.nodes.clone()
      }

      fn get_source(&self) -> Option<Source<'a>> {
        self.source.clone()
      }

      fn set_source(&mut self, source: Source<'a>) {
        self.source = Some(source);
      }

      fn as_raws(&self) -> &dyn RawBefore {
        &self.raws as &dyn RawBefore
      }

      fn as_raws_mut(&mut self) -> &mut dyn RawBefore {
        &mut self.raws as &mut dyn RawBefore
      }

      fn as_trait(&'a self) -> &dyn NodeTrait<'a> {
        self as &dyn NodeTrait<'a>
      }
    }
  };
}
