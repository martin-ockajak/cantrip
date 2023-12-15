use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use crate::extensions::Map;

impl<K, V> Map<K, V> for HashMap<K, V> {
  type Root<X, Y> = HashMap<X, Y>;

  fn all(&self, predicate: impl FnMut((&K, &V)) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl FnMut((&K, &V)) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn count_by(&self, mut predicate: impl FnMut((&K, &V)) -> bool) -> usize {
    self.iter().filter(|&x| predicate(x)).count()
  }

  fn filter_map<L, W>(&self, function: impl FnMut((&K, &V)) -> Option<(L, W)>) -> Self::Root<L, W>
  where
    K: Eq + Hash,
    L: Eq + Hash,
  {
    self.iter().filter_map(function).collect()
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
