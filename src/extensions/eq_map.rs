use std::hash::Hash;

pub trait EqMap<Key, Value> {
  type This<K, V>;

  fn filter_map<L: Eq + Hash, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>;

  fn find_map<B: Eq + Hash>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>;

  fn flat_map<L: Eq + Hash, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>;

  fn map<L: Eq + Hash, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>;

  fn map_keys<L: Eq + Hash>(self, function: impl FnMut(&Key) -> L) -> Self::This<L, Value>;

  fn map_values<W: Eq + Hash>(self, function: impl FnMut(&Value) -> W) -> Self::This<Key, W>;
}
