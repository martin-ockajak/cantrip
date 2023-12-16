use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::iter::FromIterator;

pub trait EqMap<Key, Value> {
  type This<K, V>;

  fn add(self, key: Key, value: Value) -> Self
    where
      Key: Eq + Hash,
      Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  fn concat(self, iterable: impl IntoIterator<Item = (Key, Value)>) -> Self
  where
    Key: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iterable).collect()
  }

  fn diff(self, iterable: impl IntoIterator<Item = Key>) -> Self
  where
    Key: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    let mut removed: HashSet<Key> = HashSet::new();
    removed.extend(iterable);
    self.into_iter().filter(|(k, _)| !removed.contains(k)).collect()
  }

  fn exclude(self, key: &Key) -> Self
  where
    Key: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter_map(|(k, v)| if &k != key { Some((k, v)) } else { None }).collect()
  }

  fn filter(self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Self
  where
    Key: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(k, v)| predicate((k, v))).collect()
  }

  fn filter_keys(self, mut predicate: impl FnMut(&Key) -> bool) -> Self
  where
    Key: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(k, _)| predicate(k)).collect()
  }

  fn filter_map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Key: Eq + Hash,
    L: Eq + Hash;

  fn filter_values(self, mut predicate: impl FnMut(&Value) -> bool) -> Self
  where
    Key: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(_, v)| predicate(v)).collect()
  }

  fn find_map<B>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>
  where
    Key: Eq + Hash,
    B: Eq + Hash;

  fn flat_map<L, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    L: Eq + Hash,
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

  fn map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    L: Eq + Hash;

  fn map_keys<L>(self, function: impl FnMut(&Key) -> L) -> Self::This<L, Value>
  where
    Key: Eq + Hash,
    L: Eq + Hash;

  fn map_values<W>(self, function: impl FnMut(&Value) -> W) -> Self::This<Key, W>
  where
    Key: Eq + Hash,
    W: Eq + Hash;

  fn unit(key: Key, value: Value) -> Self
  where
    Key: Eq + Hash,
    Self: FromIterator<(Key, Value)>,
  {
    iter::once((key, value)).collect()
  }
}
