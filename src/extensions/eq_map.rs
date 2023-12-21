use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::iter::FromIterator;

pub trait EqMap<Key, Value> {
  type This<K, V>;

  fn filter_map<L: Eq + Hash, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>;

  fn filter_values(self, mut predicate: impl FnMut(&Value) -> bool) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(_, v)| predicate(v)).collect()
  }

  fn find_map<B: Eq + Hash>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>;

  fn flat_map<L: Eq + Hash, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>;

  fn intersect(self, iterable: impl IntoIterator<Item = Key>) -> Self
  where
    Key: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    let mut retained: HashSet<Key> = HashSet::new();
    retained.extend(iterable);
    self.into_iter().filter(|(k, _)| retained.contains(k)).collect()
  }

  fn map<L: Eq + Hash, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>;

  fn map_keys<L: Eq + Hash>(self, function: impl FnMut(&Key) -> L) -> Self::This<L, Value>;

  fn map_values<W: Eq + Hash>(self, function: impl FnMut(&Value) -> W) -> Self::This<Key, W>;
}
