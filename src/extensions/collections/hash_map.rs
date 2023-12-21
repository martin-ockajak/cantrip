use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use crate::extensions::*;

impl<'a, Key, Value> Test<(&'a Key, &'a Value)> for HashMap<Key, Value> {
  fn xall(&self, predicate: impl FnMut(&(&'a Key, &'a Value)) -> bool) -> bool {
    todo!()
  }
}

impl<Key, Value> Map<Key, Value> for HashMap<Key, Value> {
  type This<X, V> = HashMap<X, V>;

  #[inline]
  fn all(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    all_pairs(self.iter(), |&x| predicate(x))
  }

  #[inline]
  fn any(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    any_pairs(self.iter(), |&x| predicate(x))
  }

  #[inline]
  fn count_by(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> usize {
    count_by_pairs(self.iter(), |&x| predicate(x))
  }

  #[inline]
  fn find(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)> {
    self.iter().find(|&x| predicate(x))
  }

  #[inline]
  fn fold<B>(&self, init: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B {
    fold_pairs(self.iter(), init, function)
  }

  #[inline]
  fn max_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn min_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn reduce(&self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)> {
    reduce_pairs(self.iter(), function)
  }
}

impl<Key: Eq + Hash, Value> EqMap<Key, Value> for HashMap<Key, Value> {
  type This<X, V> = HashMap<X, V>;

  fn filter_map<L: Eq + Hash, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B: Eq + Hash>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn flat_map<L: Eq + Hash, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
  {
    self.iter().flat_map(function).collect()
  }

  fn map<L: Eq + Hash, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W> {
    self.iter().map(function).collect()
  }

  fn map_keys<L: Eq + Hash>(self, mut function: impl FnMut(&Key) -> L) -> Self::This<L, Value> {
    self.into_iter().map(|(k, v)| (function(&k), v)).collect()
  }

  fn map_values<W: Eq + Hash>(self, mut function: impl FnMut(&Value) -> W) -> Self::This<Key, W> {
    self.into_iter().map(|(k, v)| (k, function(&v))).collect()
  }
}
