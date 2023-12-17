use std::cmp::Ordering;
use std::collections::{HashMap, LinkedList};
use std::hash::Hash;

use crate::extensions::util::multi_map::MultiMap;
use crate::extensions::*;

impl<Item> Traversable<Item> for LinkedList<Item> {
  fn all(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    all(self.iter(), predicate)
  }

  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    any(self.iter(), predicate)
  }

  fn count_by(&self, predicate: impl FnMut(&Item) -> bool) -> usize {
    count_by(self.iter(), predicate)
  }

  fn find(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B {
    fold(self.iter(), init, function)
  }

  fn max_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  fn min_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  fn reduce(&self, function: impl FnMut(&Item, &Item) -> Item) -> Option<Item> {
    reduce(self.iter(), function)
  }
}

impl<Item> Ordered<Item> for LinkedList<Item> {
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  fn rfind(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().rev().find(|&x| predicate(x))
  }

  fn rfold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().rfold(init, function)
  }

  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().rposition(predicate)
  }
}

impl<Item> Aggregable<Item> for LinkedList<Item> {}

impl<Item> Collectible<Item> for LinkedList<Item> {
  type This<I> = LinkedList<I>;
}

impl<Item> Sequence<Item> for LinkedList<Item> {
  type This<I> = LinkedList<I>;

  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(function).collect()
  }

  fn grouped_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    K: Eq + Hash,
    Self: Sized,
  {
    HashMap::group_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }

  fn init(self) -> Self {
    let mut iterator = self.into_iter().rev();
    iterator.next();
    iterator.rev().collect()
  }

  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B> {
    self.iter().map(function).collect()
  }

  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B> {
    self.iter().map_while(predicate).collect()
  }

  fn rev(self) -> Self {
    self.into_iter().rev().collect()
  }

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B> {
    self.iter().scan(init, function).collect()
  }
}
