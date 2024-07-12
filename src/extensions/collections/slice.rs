use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use crate::extensions::*;

impl<Item> Traversable<Item> for [Item] {
  #[inline]
  fn all(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    all(self.iter(), predicate)
  }

  #[inline]
  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    any(self.iter(), predicate)
  }

  #[inline]
  fn count_by(&self, predicate: impl FnMut(&Item) -> bool) -> usize {
    count_by(self.iter(), predicate)
  }

  #[inline]
  fn find(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().find(|&x| predicate(x))
  }

  #[inline]
  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  #[inline]
  fn fold<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().fold(initial_value, function)
  }

  #[inline]
  fn group_fold<K, B>(
    &self, to_key: impl FnMut(&Item) -> K, initial_value: B, function: impl FnMut(B, &Item) -> B,
  ) -> HashMap<K, B>
  where
    K: Eq + Hash,
    B: Clone,
  {
    group_fold(self.iter(), to_key, initial_value, function)
  }

  #[inline]
  fn max_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn max_by_key<B: Ord>(&self, mut to_key: impl FnMut(&Item) -> B) -> Option<&Item> {
    self.iter().max_by_key(|&x| to_key(x))
  }

  #[inline]
  fn min_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn min_by_key<B: Ord>(&self, mut to_key: impl FnMut(&Item) -> B) -> Option<&Item> {
    self.iter().min_by_key(|&x| to_key(x))
  }

  #[inline]
  fn minmax_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<(&Item, &Item)> {
    minmax_by(self.iter(), compare)
  }

  #[inline]
  fn minmax_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<(&Item, &Item)> {
    minmax_by_key(self.iter(), to_key)
  }

  #[inline]
  fn reduce(&self, function: impl FnMut(&Item, &Item) -> Item) -> Option<Item> {
    reduce(self.iter(), function)
  }

  #[inline]
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    subset(self.iter(), elements)
  }

  #[inline]
  fn superset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    superset(self.iter(), elements)
  }
}

impl<Item> Ordered<Item> for [Item] {
  #[inline]
  fn common_prefix_length<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> usize
  where
    Item: PartialEq + 'a,
  {
    common_prefix_length(self.iter(), elements)
  }
  #[inline]
  fn common_suffix_length<'a, I>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item, Iterator<'a> = I>) -> usize
  where
    I: DoubleEndedIterator<Item = &'a Item>,
    Item: PartialEq + 'a
  {
    common_suffix_length(self.iter().rev(), elements)
  }

  #[inline]
  fn count_unique(&self) -> usize
  where
    Item: Eq + Hash,
  {
    count_unique(self.iter())
  }

  #[inline]
  fn equivalent<'a>(&'a self, iterable: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    equivalent(self.iter(), iterable)
  }

  #[inline]
  fn frequencies<'a>(&'a self) -> HashMap<&'a Item, usize>
  where
    Item: Eq + Hash + 'a
  {
    frequencies(self.iter())
  }

  #[inline]
  fn frequencies_by<K: Eq + Hash>(&self, to_key: impl FnMut(&Item) -> K) -> HashMap<K, usize> {
    frequencies_by(self.iter(), to_key)
  }

  #[inline]
  fn joined(&self, separator: &str) -> String
  where
    Item: Display,
  {
    joined(self.iter(), separator)
  }

  #[inline]
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  #[inline]
  fn positions(&self, predicate: impl FnMut(&Item) -> bool) -> Vec<usize> {
    positions(self.iter(), predicate)
  }

  #[inline]
  fn position_sequence<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Option<usize>
  where
    Item: PartialEq + 'a,
  {
    position_sequence(self.iter(), elements)
  }

  #[inline]
  fn rfind(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().rev().find(|&x| predicate(x))
  }

  #[inline]
  fn rfold<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().rfold(initial_value, function)
  }

  #[inline]
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().rposition(predicate)
  }
}

impl<Item> Slice<Item> for [Item] {
  #[inline]
  fn init(&self) -> &Self {
    &self[0..max(self.len() - 1, 0)]
  }

  #[inline]
  fn skip(&self, n: usize) -> &Self {
    &self[min(n, self.len())..self.len()]
  }

  #[inline]
  fn skip_while(&self, mut predicate: impl FnMut(&Item) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[min(index, self.len())..self.len()],
      None => &self[0..0],
    }
  }

  #[inline]
  fn tail(&self) -> &Self {
    &self[min(1, self.len())..self.len()]
  }

  #[inline]
  fn take(&self, n: usize) -> &Self {
    &self[0..min(n, self.len())]
  }

  #[inline]
  fn take_while(&self, mut predicate: impl FnMut(&Item) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[0..min(index, self.len())],
      None => self,
    }
  }
}
