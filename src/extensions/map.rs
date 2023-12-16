use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

pub trait Map<Key, Value> {
  type This<K, V>;

  fn add(self, key: Key, value: Value) -> Self
  where
    Key: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  fn all(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

  fn count_by(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> usize;

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

  fn find(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)>;

  fn find_map<B>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>
  where
    Key: Eq + Hash,
    B: Eq + Hash;

  fn flat_map<L, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    L: Eq + Hash,
    R: IntoIterator<Item = (L, W)>;

  fn fold<B>(&self, init: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B;

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

  fn max_by(&self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  fn min_by(&self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  fn partition(self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> (Self, Self)
  where
    Self: Sized + Default + Extend<(Key, Value)> + IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().partition(|(k, v)| predicate((&k, &v)))
  }

  fn product_keys<S>(self) -> Key
  where
    Key: Product,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(k, _)| k).product()
  }

  fn product_values<S>(self) -> Value
  where
    Value: Product,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(_, v)| v).product()
  }

  fn reduce(&self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)>;

  fn sum_keys(self) -> Key
  where
    Key: Sum,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(k, _)| k).sum()
  }

  fn sum_values(self) -> Value
  where
    Value: Sum,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(_, v)| v).sum()
  }

  fn unit(key: Key, value: Value) -> Self
  where
    Key: Eq + Hash,
    Self: FromIterator<(Key, Value)>,
  {
    iter::once((key, value)).collect()
  }
}

pub(crate) fn all_pair<A>(mut iterator: impl Iterator<Item = A>, mut predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.all(|x| predicate(&x))
}

pub(crate) fn any_pair<A>(mut iterator: impl Iterator<Item = A>, mut predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.any(|x| predicate(&x))
}

pub(crate) fn count_by_pair<A>(iterator: impl Iterator<Item = A>, predicate: impl FnMut(&A) -> bool) -> usize {
  iterator.filter(predicate).count()
}

pub(crate) fn fold_pair<A, B>(iterator: impl Iterator<Item = A>, init: B, mut function: impl FnMut(B, &A) -> B) -> B {
  iterator.fold(init, |r, x| function(r, &x))
}

pub(crate) fn reduce_pair<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut((&K, &V), (&K, &V)) -> (K, V),
) -> Option<(K, V)> {
  iterator.next().and_then(|value1| {
    iterator.next().map(|value2| iterator.fold(function(value1, value2), |r, x| function((&r.0, &r.1), x)))
  })
}
