use crate::extensions::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::Hash;

impl<Key: Ord, Value> Map<Key, Value> for BTreeMap<Key, Value> {
  type This<X, V> = BTreeMap<X, V>;

  #[inline]
  fn all(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    self.iter().all(predicate)
  }

  #[inline]
  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    self.iter().any(predicate)
  }

  #[inline]
  fn count_by(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> usize {
    self.iter().filter(|&x| predicate(x)).count()
  }

  #[inline]
  fn count_unique(&self) -> usize
  where
    Value: Eq + Hash,
  {
    count_unique(self.values())
  }

  #[inline]
  fn disjoint<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a,
  {
    disjoint(self.keys(), elements)
  }

  #[inline]
  fn filter_map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.iter().filter_map(function).collect()
  }

  #[inline]
  fn find(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)> {
    self.iter().find(|&x| predicate(x))
  }

  #[inline]
  fn find_map<B>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  #[inline]
  fn flat_map<L, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.iter().flat_map(function).collect()
  }

  #[inline]
  fn fold<B>(self, initial_value: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B {
    self.iter().fold(initial_value, function)
  }

  #[inline]
  fn for_each(&self, function: impl FnMut((&Key, &Value))) {
    self.iter().for_each(function)
  }

  #[inline]
  fn map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.iter().map(function).collect()
  }

  #[inline]
  fn max_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn max_by_key<K>(&self, mut to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>
  where
    K: Ord,
  {
    self.iter().max_by_key(|&x| to_key(x))
  }

  #[inline]
  fn min_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn min_by_key<K>(&self, mut to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>
  where
    K: Ord,
  {
    self.iter().min_by_key(|&x| to_key(x))
  }

  #[inline]
  fn minmax_by(
    &self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering,
  ) -> Option<((&Key, &Value), (&Key, &Value))> {
    minmax_by_pairs(self.iter(), compare)
  }

  #[inline]
  fn minmax_by_key<K>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<((&Key, &Value), (&Key, &Value))>
  where
    K: Ord,
  {
    minmax_by_key_pairs(self.iter(), to_key)
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
  fn reduce(self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)> {
    reduce_pairs(self.iterator(), function)
  }

  #[inline]
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a,
  {
    subset(self.keys(), elements)
  }

  #[inline]
  fn superset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a,
  {
    superset(self.keys(), elements)
  }
}
