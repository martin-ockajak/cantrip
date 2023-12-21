use std::collections::BTreeSet;
use std::iter;
use std::iter::FromIterator;

pub trait OrdMap<Key, Value> {
  type This<K, V>;

  fn filter_map<L: Ord, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>;

  fn filter_values(self, mut predicate: impl FnMut(&Value) -> bool) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(_, v)| predicate(v)).collect()
  }

  fn find_map<B: Ord>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>;

  fn flat_map<L: Ord, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>;

  fn intersect(self, iterable: impl IntoIterator<Item = Key>) -> Self
  where
    Key: Ord,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    let mut retained: BTreeSet<Key> = BTreeSet::new();
    retained.extend(iterable);
    self.into_iter().filter(|(k, _)| retained.contains(k)).collect()
  }

  fn map<L: Ord, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>;

  fn map_keys<L: Ord>(self, function: impl FnMut(&Key) -> L) -> Self::This<L, Value>;

  fn map_values<W: Ord>(self, function: impl FnMut(&Value) -> W) -> Self::This<Key, W>;
}
