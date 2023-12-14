use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

use crate::extensions::Map;

impl<K, V> Map<K, V> for HashMap<K, V> {
  type Root<X, Y> = HashMap<X, Y>;

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

  fn count_by(&self, mut predicate: impl FnMut((&K, &V)) -> bool) -> usize {
    self.iter().filter(|&x| predicate(x)).count()
  }

  fn concat(self, iterable: impl IntoIterator<Item = (K, V)>) -> Self
  where
    K: Eq + Hash,
  {
    self.into_iter().chain(iterable.into_iter()).collect()
  }

  fn delete(self, key: &K) -> Self
  where
    K: Eq + Hash,
  {
    self.into_iter().filter_map(|(k, v)| if &k != key { Some((k, v)) } else { None }).collect()
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

  fn filter_map<L, W>(&self, function: impl FnMut((&K, &V)) -> Option<(L, W)>) -> Self::Root<L, W>
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

  fn flat_map<L, W, R>(&self, function: impl FnMut((&K, &V)) -> R) -> Self::Root<L, W>
  where
    L: Eq + Hash,
    R: IntoIterator<Item = (L, W)>,
  {
    self.iter().flat_map(function).collect()
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

  fn map<L, W>(&self, function: impl FnMut((&K, &V)) -> (L, W)) -> Self::Root<L, W>
  where
    L: Eq + Hash,
  {
    self.iter().map(function).collect()
  }

  fn map_keys<L>(self, mut function: impl FnMut(&K) -> L) -> Self::Root<L, V>
  where
    K: Eq + Hash,
    L: Eq + Hash,
  {
    self.into_iter().map(|(k, v)| (function(&k), v)).collect()
  }

  fn map_values<W>(self, mut function: impl FnMut(&V) -> W) -> Self::Root<K, W>
  where
    K: Eq + Hash,
    W: Eq + Hash,
  {
    self.into_iter().map(|(k, v)| (k, function(&v))).collect()
  }

  fn max_by(&self, mut compare: impl FnMut((&K, &V), (&K, &V)) -> Ordering) -> Option<(&K, &V)> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  fn min_by(&self, mut compare: impl FnMut((&K, &V), (&K, &V)) -> Ordering) -> Option<(&K, &V)> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  fn product_keys<S>(self) -> K
  where
    K: Product,
  {
    self.into_iter().map(|(k, _)| k).product()
  }

  fn product_values<S>(self) -> V
  where
    V: Product,
  {
    self.into_iter().map(|(_, v)| v).product()
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

  fn sum_keys(self) -> K
  where
    K: Sum,
  {
    self.into_iter().map(|(k, _)| k).sum()
  }

  fn sum_values(self) -> V
  where
    V: Sum,
  {
    self.into_iter().map(|(_, v)| v).sum()
  }

  fn unit(key: K, value: V) -> Self
  where
    K: Eq + Hash,
  {
    iter::once((key, value)).collect()
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::extensions::*;

  #[quickcheck]
  fn map(data: HashMap<i32, i32>) -> bool {
    let function = |(k, v): (&i32, &i32)| (*k, *v as i64);
    let result = data.map(function);
    let expected = data.iter().map(function).collect::<HashMap<i32, i64>>();
    result == expected
  }
}
