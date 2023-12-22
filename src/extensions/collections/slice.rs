use std::cmp::{max, min, Ordering};

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

impl<Item> Reversible<Item> for [Item] {
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

impl<Item> Slice<Item> for [Item] {
  fn init(&self) -> &Self {
    &self[0..max(self.len() - 1, 0)]
  }

  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  fn skip_while(&self, mut predicate: impl FnMut(&Item) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[min(index, self.len())..self.len()],
      None => &self[0..0],
    }
  }

  fn tail(&self) -> &Self {
    &self[min(1, self.len())..self.len()]
  }

  fn take_while(&self, mut predicate: impl FnMut(&Item) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[0..min(index, self.len())],
      None => &self,
    }
  }
}
