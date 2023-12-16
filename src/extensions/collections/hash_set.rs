use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::extensions::util::multimap::MultiMap;
use crate::extensions::*;

impl<Item> Traversable<Item> for HashSet<Item> {
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

impl<Item> Aggregable<Item> for HashSet<Item> {}

impl<Item> Collectible<Item> for HashSet<Item> {
  type This<I> = HashSet<I>;
}

impl<Item> Set<Item> for HashSet<Item> {
  type This<I> = HashSet<I>;

  fn exclude(self, value: &Item) -> Self
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().filter(|x| x != value).collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Item: Eq + Hash,
    B: Eq + Hash,
  {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>
  where
    Item: Eq + Hash,
    B: Eq + Hash,
  {
    self.iter().find_map(function)
  }

  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    B: Eq + Hash,
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(function).collect()
  }

  fn group_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    Item: Eq + Hash,
    K: Eq + Hash,
    Self: Sized,
  {
    HashMap::from_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }

  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    B: Eq + Hash,
  {
    self.iter().map(function).collect()
  }
}
