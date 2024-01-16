use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::Display;

use crate::extensions::*;

impl<Key: Ord, Value> Map<Key, Value> for BTreeMap<Key, Value> {
  type This<X, V> = BTreeMap<X, V>;

  #[inline]
  fn all(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
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
  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    any_pairs(self.iter(), predicate)
  }

  #[inline]
  fn count_by(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> usize {
    count_by_pairs(self.iter(), predicate)
  }

  #[inline]
  fn filter_map<L, W>(&self, mut function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    filter_map_pairs(self.iter(), |&x| function(x))
  }

  #[inline]
  fn find(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)> {
    self.iter().find(|&x| predicate(x))
  }

  #[inline]
  fn find_map<B>(&self, mut function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B> {
    find_map_pairs(self.iter(), |&x| function(x))
  }

  #[inline]
  fn flat_map<L, W, R>(&self, mut function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    flat_map_pairs(self.iter(), |&x| function(x))
  }

  #[inline]
  fn fold<B>(&self, init: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B {
    self.iter().fold(init, function)
  }

  #[inline]
  fn join_items(&self, separator: &str) -> String
  where
    Key: Display,
    Value: Display,
  {
    join_items_pairs(self.iter(), separator)
  }

  #[inline]
  fn map<L, W>(&self, mut function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    map_pairs(self.iter(), |&x| function(x))
  }

  #[inline]
  fn max_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn max_by_key<K: Ord>(&self, mut to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)> {
    self.iter().max_by_key(|&x| to_key(x))
  }

  #[inline]
  fn min_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn min_by_key<K: Ord>(&self, mut to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)> {
    self.iter().min_by_key(|&x| to_key(x))
  }

  #[inline]
  fn minmax_by(
    &self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering,
  ) -> Option<((&Key, &Value), (&Key, &Value))> {
    minmax_by_pairs(self.iter(), compare)
  }

  #[inline]
  fn minmax_by_key<K: Ord>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<((&Key, &Value), (&Key, &Value))> {
    minmax_by_key_pairs(self.iter(), to_key)
  }

  #[inline]
  fn reduce(&self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)> {
    reduce_pairs(self.iter(), function)
  }

  #[inline]
  fn partition_map<L1, W1, L2, W2>(
    &self, function: impl FnMut((&Key, &Value)) -> Result<(L1, W1), (L2, W2)>,
  ) -> (Self::This<L1, W1>, Self::This<L2, W2>)
  where
    Self::This<L1, W1>: Default + Extend<(L1, W1)>,
    Self::This<L2, W2>: Default + Extend<(L2, W2)>,
  {
    partition_map_pairs(self.iter(), function)
  }

  #[inline]
  fn scan<S, L, W>(
    self, initial_state: S, function: impl FnMut(&mut S, (&Key, &Value)) -> Option<(L, W)>,
  ) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.iter().scan(initial_state, function).collect()
  }
}
