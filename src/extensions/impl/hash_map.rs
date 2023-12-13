use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use crate::extensions::api::map::{MapFunctor, MapIterable, MapMonad};

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

}
