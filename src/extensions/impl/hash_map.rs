use crate::extensions::api::map::{MapFunctor, MapMonad};
use crate::extensions::MapOps;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;

impl<K, V> MapFunctor<K, V> for HashMap<K, V> {
  type C<X, Y> = HashMap<X, Y>;

  fn map<L, W>(&self, function: impl FnMut((&K, &V)) -> (L, W)) -> Self::C<L, W>
  where
    L: Eq + Hash,
  {
    self.iter().map(function).collect()
  }
}

impl<K, V> MapMonad<K, V> for HashMap<K, V> {
  type C<X, Y> = HashMap<X, Y>;

  fn unit(key: K, value: V) -> Self::C<K, V>
  where
    K: Eq + Hash,
  {
    iter::once((key, value)).collect()
  }

  fn flat_map<L, W, R>(&self, function: impl FnMut((&K, &V)) -> R) -> Self::C<L, W>
  where
    L: Eq + Hash,
    R: IntoIterator<Item = (L, W)>,
  {
    self.iter().flat_map(function).collect()
  }
}

impl<K, V> MapOps<K, V> for HashMap<K, V> {
  type C<X, Y> = HashMap<X, Y>;

  fn add(self, key: K, value: V) -> Self
  where
    K: Eq + Hash,
  {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  fn all(&self, predicate: impl FnMut((&K, &V)) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl FnMut((&K, &V)) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn delete(self, key: &K) -> Self
  where
    K: Eq + Hash,
  {
    self.into_iter().filter_map(|(k, v)| if &k == key { Some((k, v)) } else { None }).collect()
  }

  fn diff(self, iterable: impl IntoIterator<Item = K>) -> Self
  where
    K: Eq + Hash,
  {
    let mut removed: HashSet<K> = HashSet::new();
    removed.extend(iterable.into_iter());
    self.into_iter().filter(|(k, _)| !removed.contains(k)).collect()
  }

  fn filter(self, mut predicate: impl FnMut((&K, &V)) -> bool) -> Self
  where
    K: Eq + Hash,
  {
    self.into_iter().filter(|(k, v)| predicate((k, v))).collect()
  }

  fn filter_keys(self, mut predicate: impl FnMut(&K) -> bool) -> Self
  where
    K: Eq + Hash,
  {
    self.into_iter().filter(|(k, _)| predicate(k)).collect()
  }

  fn filter_map<L, W>(&self, function: impl FnMut((&K, &V)) -> Option<(L, W)>) -> Self::C<L, W>
  where
    K: Eq + Hash,
    L: Eq + Hash,
  {
    self.iter().filter_map(function).collect()
  }

  fn filter_values(self, mut predicate: impl FnMut(&V) -> bool) -> Self
  where
    K: Eq + Hash,
  {
    self.into_iter().filter(|(_, v)| predicate(v)).collect()
  }

  fn find(&self, mut predicate: impl FnMut((&K, &V)) -> bool) -> Option<(&K, &V)> {
    self.iter().find(|&x| predicate(x))
  }

  fn find_map<B>(&self, function: impl FnMut((&K, &V)) -> Option<B>) -> Option<B>
  where
    K: Eq + Hash,
    B: Eq + Hash,
  {
    self.iter().find_map(function)
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, (&K, &V)) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn intersect(self, iterable: impl IntoIterator<Item = K>) -> Self
    where
      K: Eq + Hash,
  {
    let mut retained: HashSet<K> = HashSet::new();
    retained.extend(iterable.into_iter());
    self.into_iter().filter(|(k, _)| retained.contains(k)).collect()
  }

  fn map_keys<L>(self, mut function: impl FnMut(&K) -> L) -> Self::C<L, V>
  where
    K: Eq + Hash,
    L: Eq + Hash,
  {
    self.into_iter().map(|(k, v)| (function(&k), v)).collect()
  }

  fn map_values<W>(self, mut function: impl FnMut(&V) -> W) -> Self::C<K, W>
  where
    K: Eq + Hash,
    W: Eq + Hash,
  {
    self.into_iter().map(|(k, v)| (k, function(&v))).collect()
  }

  fn merge(self, iterable: impl IntoIterator<Item = (K, V)>) -> Self
  where
    K: Eq + Hash,
  {
    self.into_iter().chain(iterable.into_iter()).collect()
  }

  fn reduce(&self, mut function: impl FnMut((&K, &V), (&K, &V)) -> (K, V)) -> Option<(K, V)> {
    let mut iterator = self.iter();
    match iterator.next() {
      Some(value1) => match iterator.next() {
        Some(value2) => Some(iterator.fold(function(value1, value2), |r, x| function((&r.0, &r.1), x))),
        _ => None,
      },
      _ => None,
    }
  }

  fn rfold<B>(&self, init: B, mut function: impl FnMut(B, (&K, &V)) -> B) -> B {
    let values = self.iter().collect::<Vec<(&K, &V)>>();
    values.iter().rfold(init, |r, &x| function(r, x))
  }
}

#[cfg(test)]
mod tests {
  use crate::extensions::*;
  use std::collections::HashMap;

  #[quickcheck]
  fn test_map_hash_map(data: HashMap<i32, i32>) -> bool {
    let function = |(k, v): (&i32, &i32)| (*k, *v as i64);
    let result = data.map(function);
    let expected = data.iter().map(function).collect::<HashMap<i32, i64>>();
    result == expected
  }
}
