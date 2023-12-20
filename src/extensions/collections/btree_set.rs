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
  fn min_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }
}

impl<Item> Summable<Item> for BTreeSet<Item> {}

impl<Item> Productable<Item> for BTreeSet<Item> {}

impl<Item> Collectible<Item> for BTreeSet<Item> {
  type This<I> = BTreeSet<I>;
}

impl<Item: Ord> OrdSet<Item> for BTreeSet<Item> {
  type This<I> = BTreeSet<I>;

  fn exclude(self, value: &Item) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().filter(|x| x != value).collect()
  }

  fn filter_map<B: Ord>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B: Ord>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn flat_map<B: Ord, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(function).collect()
  }

  fn map<B: Ord>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B> {
    self.iter().map(function).collect()
  }
}
