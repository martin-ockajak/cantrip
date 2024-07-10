use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::hash::Hash;
use crate::extensions::*;

impl<Item> Traversable<Item> for BinaryHeap<Item> {
  #[inline]
  fn all(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    all(self.iter(), predicate)
  }

  #[inline]
  fn all_equal(&self) -> bool
  where
    Item: PartialEq,
  {
    all_equal(self.iter())
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
  fn includes<'a>(&'a self, iterable: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    includes(self.iter(), iterable)
  }

  #[inline]
  fn join_items(&self, separator: &str) -> String
  where
    Item: Display,
  {
    join_items(self.iter(), separator)
  }

  #[inline]
  fn max_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn max_by_key<K: Ord>(&self, mut to_key: impl FnMut(&Item) -> K) -> Option<&Item> {
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
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    subset(self.iter(), elements)
  }
}

impl<Item: Ord> Collectible<Item> for BinaryHeap<Item> {
  type This<I> = BinaryHeap<I>;

  #[inline]
  fn combinations(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
  {
    combinations(self.iter(), k)
  }

  #[inline]
  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    self.iter().filter_map(function).collect()
  }

  #[inline]
  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self::This<B>: FromIterator<B>,
  {
    self.iter().flat_map(function).collect()
  }

  #[inline]
  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    self.iter().map(function).collect()
  }

  #[inline]
  fn partition_map<A, B>(&self, function: impl FnMut(&Item) -> Result<A, B>) -> (Self::This<A>, Self::This<B>)
  where
    Self::This<A>: Default + Extend<A>,
    Self::This<B>: Default + Extend<B>,
  {
    partition_map(self.iter(), function)
  }

  #[inline]
  fn powerset(&self) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized
  {
    powerset(self.iter())
  }

  #[inline]
  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    self.iter().scan(init, function).collect()
  }
}
