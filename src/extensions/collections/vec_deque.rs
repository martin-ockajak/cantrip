use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Display;
use std::hash::Hash;

use crate::extensions::*;

impl<Item> Traversable<Item> for VecDeque<Item> {
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
  fn min_by_key<K: Ord>(&self, mut to_key: impl FnMut(&Item) -> K) -> Option<&Item> {
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
}

impl<Item> Reversible<Item> for VecDeque<Item> {
  #[inline]
  fn rfind(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().rev().find(|&x| predicate(x))
  }

  #[inline]
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().rposition(predicate)
  }
}

impl<Item> Collectible<Item> for VecDeque<Item> {
  type This<I> = VecDeque<I>;

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
  fn powerset(self) -> Vec<Self>
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

impl<Item> Sequence<Item> for VecDeque<Item> {
  type This<I> = VecDeque<I>;

  #[inline]
  fn all_unique(&self) -> bool
  where
    Item: Eq + Hash,
  {
    all_unique(self.iter())
  }

  #[inline]
  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B> {
    self.iter().map_while(predicate).collect()
  }

  #[inline]
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  #[inline]
  fn positions(&self, predicate: impl FnMut(&Item) -> bool) -> Self::This<usize> {
    positions(self.iter(), predicate)
  }

  #[inline]
  fn windowed(&self, size: usize) -> Self::This<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
    Self::This<Self>: FromIterator<Self>,
  {
    windowed(self.iter(), size)
  }
}
