use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use crate::extensions::api::map::{MapFunctor, MapIterable, MapMonad};
use crate::extensions::MapCollection;

impl<K, V, L: Eq + Hash, W> MapFunctor<K, V, L, W> for HashMap<K, V> {
  type C<X, Y> = HashMap<X, Y>;

  fn map(&self, function: impl Fn((&K, &V)) -> (L, W)) -> Self::C<L, W> {
    self.iter().map(function).collect()
  }
}

impl<K, V, L: Eq + Hash, W> MapMonad<K, V, L, W> for HashMap<K, V> {
  type C<X, Y> = HashMap<X, Y>;

  fn unit(key: K, value: V) -> Self::C<K, V> where K: Eq + Hash {
    iter::once((key, value)).collect()
  }

  fn flat_map(&self, function: impl Fn((&K, &V)) -> Self::C<L, W>) -> Self::C<L, W> {
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

  fn reduce(&self, function: impl Fn((&K, &V), (&K, &V)) -> (K, V)) -> Option<(K, V)> where K: Clone, V: Clone {
    let mut iterator = self.iter();
    iterator.next().and_then(|head| {
      let init = (head.0.clone(), head.1.clone());
      Some(iterator.fold(init, |r, x| function((&r.0, &r.1), x)))
    })
  }

  fn rfold<B>(&self, init: B, function: impl Fn(B, (&K, &V)) -> B) -> B {
    let values = self.iter().collect::<Vec<(&K, &V)>>();
    values.iter().rfold(init, |r, &x| function(r, x))
  }
}

impl<K: Eq + Hash + Clone, V: Clone> MapCollection<K, V> for HashMap<K, V> {
  type C<X> = Vec<X>;

  fn add(&self, key: K, value: V) -> Self {
    let mut result = self.clone();
    result.insert(key, value);
    result
  }

  fn delete(&self, key: &K) -> Self {
    let mut result = self.clone();
    result.remove(key);
    result
  }

  fn diff(&self, iterable: &(impl IntoIterator<Item = K> + Clone)) -> Self {
    let mut result = self.clone();
    for item in iterable.clone().into_iter() {
      result.remove(&item);
    }
    result
  }

  fn filter(&self, predicate: impl Fn((&K, &V)) -> bool) -> Self {
    self.iter().filter(|&x| predicate(x)).map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  fn filter_map<B: Eq + Hash>(&self, function: impl Fn((&K, &V)) -> Option<B>) -> Self::C<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B: Eq + Hash>(&self, function: impl Fn((&K, &V)) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn merge(&self, iterable: &(impl IntoIterator<Item = (K, V)> + Clone)) -> Self {
    let mut result = self.clone();
    result.extend(iterable.clone().into_iter());
    result
  }
}

// pub fn add_map<K, V>(values: &HashMap<K, V>, key: &K, value: &V) -> HashMap<K, V>
// where
//   K: Clone + Eq + Hash,
//   V: Clone,
// {
//   values.iter().map(|(k, v)| (k.clone(), v.clone())).chain([(key.clone(), value.clone())].into_iter()).collect()
// }
//
// pub fn remove_all_map<K, V>(values: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
//   where
//     K: Clone + Eq + Hash,
//     V: Clone,
// {
//   values.iter().filter(|(k, _)| !keys.contains(k)).map(|(k, v)| (k.clone(), v.clone())).collect()
// }
//
// pub fn remove_map<K, V>(values: &HashMap<K, V>, key: &K) -> HashMap<K, V>
//   where
//     K: Clone + Eq + Hash,
//     V: Clone,
// {
//   values.iter().filter(|(k, _)| k != &key).map(|(k, v)| (k.clone(), v.clone())).collect()
// }
//
// pub fn merge_map<K, V>(values1: &HashMap<K, V>, values2: &HashMap<K, V>) -> HashMap<K, V>
//   where
//     K: Clone + Eq + Hash,
//     V: Clone,
// {
//   values1.iter().chain(values2.iter()).map(|(k, v)| (k.clone(), v.clone())).collect()
// }

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::extensions::*;

  #[quickcheck]
  fn test_map_hash_map(data: HashMap<i32, i32>) -> bool {
    let function = |(k, v): (&i32, &i32)| (*k, *v as i64);
    let result = data.map(function);
    let expected = data.iter().map(function).collect::<HashMap<i32, i64>>();
    result == expected
  }
}
