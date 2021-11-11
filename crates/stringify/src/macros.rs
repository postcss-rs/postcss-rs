#[macro_export]
macro_rules! get_raw_value {
  ($node: ident, $prop: ident) => {
    match &$node.raws.$prop {
      Some(raw) => {
        let v = &*$node.$prop;
        if *raw.value == *v {
          &raw.raw
        } else {
          v
        }
      }
      None => &$node.$prop,
    }
  };
}