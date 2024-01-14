use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::extensions::*;

impl<Item> Traversable<Item> for BTreeSet<Item> {
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
  fn fold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B {
    fold(self.iter(), init, function)
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
    self.iter().max_by_key(|&x| to_key(x))
  }
}

impl<Item: Ord> Collectible<Item> for BTreeSet<Item> {
  type This<I> = BTreeSet<I>;

  #[inline]
  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    filter_map(self.iter(), function)
  }

  #[inline]
  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B> {
    find_map(self.iter(), function)
  }

  #[inline]
  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
    where
      R: IntoIterator<Item = B>,
      Self::This<B>: FromIterator<B>,
  {
    flat_map(self.iter(), function)
  }

  #[inline]
  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    map(self.iter(), function)
  }
}
