use std::cmp::min;
use std::hash::Hash;

use crate::Iterable;
#[allow(clippy::wildcard_imports)]
use crate::extensions::*;

impl<Item> Collection<Item> for [Item] {}

impl<Item> Sequence<Item> for [Item] {
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
    Item: PartialEq + 'a,
  {
    common_suffix_length(self.iter().rev(), elements)
  }

  #[inline]
  fn equivalent<'a>(&'a self, iterable: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    equivalent(self.iter(), iterable)
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
  fn rfold_ref<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().rfold(initial_value, function)
  }

  #[inline]
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().rposition(predicate)
  }
}

impl<Item> Slice<Item> for [Item] {
  #[inline]
  fn init_ref(&self) -> &Self {
    &self[0..self.len().saturating_sub(1)]
  }

  #[inline]
  fn skip_ref(&self, n: usize) -> &Self {
    &self[min(n, self.len())..self.len()]
  }

  #[inline]
  fn skip_while_ref(&self, mut predicate: impl FnMut(&Item) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[min(index, self.len())..self.len()],
      None => &self[0..0],
    }
  }

  #[inline]
  fn tail_ref(&self) -> &Self {
    &self[min(1, self.len())..self.len()]
  }

  #[inline]
  fn take_ref(&self, n: usize) -> &Self {
    &self[0..min(n, self.len())]
  }

  #[inline]
  fn take_while_ref(&self, mut predicate: impl FnMut(&Item) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[0..min(index, self.len())],
      None => self,
    }
  }
}
