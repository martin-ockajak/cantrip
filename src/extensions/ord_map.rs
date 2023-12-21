use std::collections::BTreeSet;
use std::iter;
use std::iter::FromIterator;

pub trait OrdMap<Key, Value> {
  type This<K, V>;

  fn filter_map<L: Ord, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>;

  fn find_map<B: Ord>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>;

  fn flat_map<L: Ord, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>;

  fn map<L: Ord, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>;

  fn map_keys<L: Ord>(self, function: impl FnMut(&Key) -> L) -> Self::This<L, Value>;

  fn map_values<W: Ord>(self, function: impl FnMut(&Value) -> W) -> Self::This<Key, W>;
}
