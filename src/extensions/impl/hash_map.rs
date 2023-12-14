use crate::extensions::api::map::{MapFunctor, MapIterable, MapMonad};
use crate::extensions::MapCollection;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;

impl<K, V, L, W> MapFunctor<K, V, L, W> for HashMap<K, V> {
  type C<X, Y> = HashMap<X, Y>;

  fn map(&self, function: impl Fn((&K, &V)) -> (L, W)) -> Self::C<L, W>
  where
    L: Eq + Hash,
  {
    self.iter().map(function).collect()
  }
}

impl<K, V, L, W> MapMonad<K, V, L, W> for HashMap<K, V> {
  type C<X, Y> = HashMap<X, Y>;

  fn unit(key: K, value: V) -> Self::C<K, V>
  where
    K: Eq + Hash,
  {
    iter::once((key, value)).collect()
  }

  fn flat_map<R>(&self, function: impl Fn((&K, &V)) -> R) -> Self::C<L, W>
  where
    R: IntoIterator<Item = (L, W)> + Clone,
    L: Eq + Hash,
  {
    self.iter().flat_map(function).collect()
  }
}

impl<K, V> MapIterable<K, V> for HashMap<K, V> {
  fn all(&self, predicate: impl Fn((&K, &V)) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl Fn((&K, &V)) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn find(&self, predicate: impl Fn((&K, &V)) -> bool) -> Option<(&K, &V)> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl Fn(B, (&K, &V)) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn reduce(&self, function: impl Fn((&K, &V), (&K, &V)) -> (K, V)) -> Option<(K, V)>
  {
    let mut iterator = self.iter();
    match iterator.next() {
      Some(value1) => {
        match iterator.next() {
          Some(value2) => {
            Some(iterator.fold(function(value1, value2), |r, x| function((&r.0, &r.1), x)))
          },
          _ => None
        }
      },
      _ => None
    }
  }

  fn rfold<B>(&self, init: B, function: impl Fn(B, (&K, &V)) -> B) -> B {
    let values = self.iter().collect::<Vec<(&K, &V)>>();
    values.iter().rfold(init, |r, &x| function(r, x))
  }
}

impl<K: Eq + Hash + Clone, V: Clone> MapCollection<K, V> for HashMap<K, V> {
  type C<X, Y> = HashMap<X, Y>;

  fn add(self, key: K, value: V) -> Self {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  fn delete(self, key: &K) -> Self {
    self.into_iter().filter_map(|(k, v)| if &k == key {
      Some((k, v))
    } else {
      None
    }).collect()
  }

  fn diff(self, iterable: impl IntoIterator<Item = K>) -> Self {
    let mut removed: HashSet<K> = HashSet::new();
    removed.extend(iterable.into_iter());
    self.into_iter().filter(|(k, _)| removed.contains(k)).collect()
  }

  fn filter(self, predicate: impl Fn((&K, &V)) -> bool) -> Self {
    self.into_iter().filter(|(k, v)| predicate((k, v))).collect()
  }

  fn filter_map<L, W>(&self, function: impl Fn((&K, &V)) -> Option<(L, W)>) -> Self::C<L, W>
  where
    L: Eq + Hash,
  {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl Fn((&K, &V)) -> Option<B>) -> Option<B>
  where
    B: Eq + Hash,
  {
    self.iter().find_map(function)
  }

  fn map_keys<L>(self, function: impl Fn(&K) -> L) -> Self::C<L, V>
  where
    L: Eq + Hash,
  {
    self.into_iter().map(|(k, v)| (function(&k), v)).collect()
  }

  fn map_values<W>(self, function: impl Fn(&V) -> W) -> Self::C<K, W>
  where
    W: Eq + Hash,
  {
    self.into_iter().map(|(k, v)| (k, function(&v))).collect()
  }

  fn merge(self, iterable: impl IntoIterator<Item = (K, V)>) -> Self {
    self.into_iter().chain(iterable.into_iter()).collect()
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
