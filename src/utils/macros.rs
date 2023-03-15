/// Macro that creates Binary Tree Map that can be converted to a surrealDB value
macro_rules! map {
  ($($k:expr => $v:expr),* $(,)?) => {{
  let mut m = ::std::collections::BTreeMap::new();
      $(m.insert($k, $v);)+
      m
  }};
}
pub(crate) use map;
