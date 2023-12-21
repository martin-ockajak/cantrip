use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use crate::extensions::*;

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
  type This<K, V> = HashMap<K, V>;

  #[inline]
  fn filter_map<L: Eq + Hash, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W> {
    self.iter().filter_map(function).collect()
  }

  #[inline]
  fn find_map<B: Eq + Hash>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  #[inline]
  fn flat_map<L: Eq + Hash, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
  {
    self.iter().flat_map(function).collect()
  }

  #[inline]
  fn map<L: Eq + Hash, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W> {
    self.iter().map(function).collect()
  }
}
