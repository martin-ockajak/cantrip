use std::cmp::Ordering;
use std::collections::HashMap;

use crate::extensions::*;

impl<Key, Value> Map<Key, Value> for HashMap<Key, Value> {
  type This<X, V> = HashMap<X, V>;

  #[inline]
  fn all(&self, predicate: impl FnMut(&(&Key, &Value)) -> bool) -> bool {
    all_pairs(self.iter(), predicate)
  }

  #[inline]
  fn all_equal(&self) -> bool
  where
    Key: PartialEq,
    Value: PartialEq,
  {
    all_equal_pairs(self.iter())
  }

  #[inline]
  fn any(&self, predicate: impl FnMut(&(&Key, &Value)) -> bool) -> bool {
    any_pairs(self.iter(), predicate)
  }

  #[inline]
  fn count_by(&self, predicate: impl FnMut(&(&Key, &Value)) -> bool) -> usize {
    count_by_pairs(self.iter(), predicate)
  }

  #[inline]
  fn filter_map<L, W>(&self, function: impl FnMut(&(&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    filter_map_pairs(self.iter(), function)
  }

  #[inline]
  fn find(&self, predicate: impl FnMut(&(&Key, &Value)) -> bool) -> Option<(&Key, &Value)> {
    self.iter().find(predicate)
  }

  #[inline]
  fn find_map<B>(&self, function: impl FnMut(&(&Key, &Value)) -> Option<B>) -> Option<B> {
    find_map_pairs(self.iter(), function)
  }

  #[inline]
  fn flat_map<L, W, R>(&self, function: impl FnMut(&(&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    flat_map_pairs(self.iter(), function)
  }

  #[inline]
  fn fold<B>(&self, init: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B {
    self.iter().fold(init, function)
  }

  #[inline]
  fn map<L, W>(&self, function: impl FnMut(&(&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    map_pairs(self.iter(), function)
  }

  #[inline]
  fn max_by(&self, compare: impl FnMut(&(&Key, &Value), &(&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().max_by(compare)
  }

  #[inline]
  fn max_by_key<K: Ord>(&self, to_key: impl FnMut(&(&Key, &Value)) -> K) -> Option<(&Key, &Value)> {
    self.iter().max_by_key(to_key)
  }

  #[inline]
  fn min_by(&self, compare: impl FnMut(&(&Key, &Value), &(&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().min_by(compare)
  }

  #[inline]
  fn min_by_key<K: Ord>(&self, to_key: impl FnMut(&(&Key, &Value)) -> K) -> Option<(&Key, &Value)> {
    self.iter().min_by_key(to_key)
  }

  #[inline]
  fn reduce(&self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)> {
    reduce_pairs(self.iter(), function)
  }
}
