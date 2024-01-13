use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;

use crate::extensions::*;

impl<Item> Traversable<Item> for Vec<Item> {
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

impl<Item> Reversible<Item> for Vec<Item> {
  #[inline]
  fn rfind(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().rev().find(|&x| predicate(x))
  }

  #[inline]
  fn rfold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().rfold(init, function)
  }

  #[inline]
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().rposition(predicate)
  }
}

impl<Item> Collectible<Item> for Vec<Item> {
  type This<I> = Vec<I>;
}

impl<Item> Sequence<Item> for Vec<Item> {
  type This<I> = Vec<I>;

  #[inline]
  fn chunked(self, chunk_size: usize) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    chunked(self.into_iter(), chunk_size)
  }

  #[inline]
  fn init(self) -> Self {
    init(self.into_iter())
  }

  #[inline]
  fn interleave(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: Default + Extend<Item>,
  {
    interleave(self.into_iter(), iterable)
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
  fn rev(self) -> Self {
    self.into_iter().rev().collect()
  }

  #[inline]
  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B> {
    self.iter().scan(init, function).collect()
  }
}

impl<Item> Indexed<Item> for Vec<Item> {
  type This<I> = Vec<I>;

  #[inline]
  fn sorted(self) -> Self
  where
    Item: Ord,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort();
    result
  }

  #[inline]
  fn sorted_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_by(compare);
    result
  }

  #[inline]
  fn sorted_unstable(self) -> Self
  where
    Item: Ord,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable();
    result
  }

  #[inline]
  fn sorted_unstable_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable_by(compare);
    result
  }
}
